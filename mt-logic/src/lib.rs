#![no_std]

mod instruction;
mod messages;

use gstd::{exec, msg, prelude::*, prog::ProgramGenerator, ActorId};
use hashbrown::HashMap;
use instruction::*;
use messages::*;
use mt_logic_io::*;
use primitive_types::H256;

const GAS_STORAGE_CREATION: u64 = 3_000_000_000;
const DELAY: u32 = 600_000;

gstd::metadata! {
    title: "Logic MultiToken contract",
    handle:
        input: MTLogicAction,
        output: MTLogicEvent,
    state:
        input: MTLogicState,
        output: MTLogicStateReply,
}

#[derive(Default)]
struct MTLogic {
    admin: ActorId,
    mtoken_id: ActorId,
    transaction_status: HashMap<H256, TransactionStatus>,
    instructions: HashMap<H256, (Instruction, Instruction)>,
    storage_code_hash: H256,
    id_to_storage: HashMap<String, ActorId>,
}

impl MTLogic {
    async fn message(&mut self, transaction_hash: H256, msg_source: &ActorId, payload: &[u8]) {
        self.assert_main_contract();

        let action = Action::decode(&mut &payload[..]).expect("Can't decode `Action`");
        let transaction_status = self
            .transaction_status
            .get(&transaction_hash)
            .unwrap_or(&TransactionStatus::InProgress);

        match transaction_status {
            // The transaction has already been made but there wasn't enough gas for a message reply
            TransactionStatus::Success => reply_ok(),
            TransactionStatus::Failure => reply_err(),
            // The transaction took place for the first time
            // Or there was not enough gas to change the `TransactionStatus`
            TransactionStatus::InProgress => {
                send_delayed_clear(transaction_hash);
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::InProgress);

                match action {
                    Action::Transfer {
                        token_id,
                        sender,
                        recipient,
                        amount,
                    } => {
                        self.transfer(
                            transaction_hash,
                            token_id,
                            &msg_source,
                            &sender,
                            &recipient,
                            amount,
                        )
                        .await
                    }
                    Action::Approve {
                        account,
                        is_approved,
                    } => {
                        self.approve(transaction_hash, &msg_source, &account, is_approved)
                            .await
                    }
                }
            }
        }
    }

    async fn transfer(
        &mut self,
        transaction_hash: H256,
        token_id: u128,
        msg_source: &ActorId,
        sender: &ActorId,
        recipient: &ActorId,
        amount: u128,
    ) {
        let sender_storage_id = self.get_or_create_storage_address(sender);
        let recipient_storage_id = self.get_or_create_storage_address(recipient);

        if recipient_storage_id == sender_storage_id {
            self.transfer_single_storage(
                transaction_hash,
                &sender_storage_id,
                token_id,
                msg_source,
                sender,
                recipient,
                amount,
            )
            .await;
            return;
        }

        let (decrease_instruction, increase_instruction) = self
            .instructions
            .entry(transaction_hash)
            .or_insert_with(|| {
                let decrease_instruction = create_decrease_instruction(
                    transaction_hash,
                    &sender_storage_id,
                    token_id,
                    &msg_source,
                    sender,
                    amount,
                );
                let increase_instruction = create_increase_instruction(
                    transaction_hash,
                    &recipient_storage_id,
                    token_id,
                    recipient,
                    amount,
                );
                (decrease_instruction, increase_instruction)
            });

        if decrease_instruction.start().await.is_err() {
            self.transaction_status
                .insert(transaction_hash, TransactionStatus::Failure);
            reply_err();
            return;
        }

        match increase_instruction.start().await {
            Err(_) => {
                if decrease_instruction.abort().await.is_ok() {
                    self.transaction_status
                        .insert(transaction_hash, TransactionStatus::Failure);
                    reply_err();
                }
            }
            Ok(_) => {
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Success);
                reply_ok();
            }
        }
    }

    async fn transfer_single_storage(
        &mut self,
        transaction_hash: H256,
        storage_id: &ActorId,
        token_id: u128,
        msg_source: &ActorId,
        sender: &ActorId,
        recipient: &ActorId,
        amount: u128,
    ) {
        let result = transfer(
            storage_id,
            transaction_hash,
            token_id,
            msg_source,
            sender,
            recipient,
            amount,
        )
        .await;

        match result {
            Ok(()) => {
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Success);
                reply_ok()
            }
            Err(()) => {
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Failure);
                reply_err();
            }
        }
    }

    async fn approve(
        &mut self,
        transaction_hash: H256,
        msg_source: &ActorId,
        account: &ActorId,
        is_approved: bool,
    ) {
        self.transaction_status
            .insert(transaction_hash, TransactionStatus::InProgress);
        let storage_id = self.get_or_create_storage_address(account);

        let result = approve(
            &storage_id,
            transaction_hash,
            msg_source,
            account,
            is_approved,
        )
        .await;

        match result {
            Ok(()) => {
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Success);
                reply_ok();
            }
            Err(()) => {
                self.transaction_status
                    .insert(transaction_hash, TransactionStatus::Failure);
                reply_err();
            }
        }
    }

    fn clear(&mut self, transaction_hash: H256) {
        self.transaction_status.remove(&transaction_hash);
    }

    fn update_storage_hash(&mut self, storage_code_hash: H256) {
        self.assert_admin();
        self.storage_code_hash = storage_code_hash;
    }

    fn get_or_create_storage_address(&mut self, address: &ActorId) -> ActorId {
        let encoded = hex::encode(address.as_ref());
        let id: String = encoded.chars().next().expect("Can't be None").to_string();
        if let Some(address) = self.id_to_storage.get(&id) {
            *address
        } else {
            let (_message_id, address) = ProgramGenerator::create_program_with_gas(
                self.storage_code_hash.into(),
                "",
                GAS_STORAGE_CREATION,
                0,
            )
            .expect("Error in creating Storage program");
            self.id_to_storage.insert(id, address);
            address
        }
    }

    async fn get_balance(&self, token_id: u128, account: &ActorId) {
        let encoded = hex::encode(account.as_ref());
        let id: String = encoded.chars().next().expect("Can't be None").to_string();

        if let Some(storage_id) = self.id_to_storage.get(&id) {
            let balance = get_balance(storage_id, token_id, account)
                .await
                .unwrap_or(0);

            msg::reply(MTLogicEvent::Balance(balance), 0)
                .expect("Error in a reply `MTLogicEvent::Balance`");
        } else {
            msg::reply(MTLogicEvent::Balance(0), 0)
                .expect("Error in a reply `MTLogicEvent::Balance`");
        }
    }

    async fn get_approval(&self, account: &ActorId, approval_target: &ActorId) {
        let encoded = hex::encode(account.as_ref());
        let id: String = encoded.chars().next().expect("Can't be None").to_string();

        if let Some(storage_id) = self.id_to_storage.get(&id) {
            let approval = get_approval(storage_id, account, approval_target)
                .await
                .unwrap_or(false);

            msg::reply(MTLogicEvent::Approval(approval), 0)
                .expect("Error in a reply `MTLogicEvent::Approval`");
        } else {
            msg::reply(MTLogicEvent::Approval(false), 0)
                .expect("Error in a reply `MTLogicEvent::Approval`");
        }
    }

    fn assert_main_contract(&self) {
        assert_eq!(
            self.mtoken_id,
            msg::source(),
            "Only main multitoken contract can send that message"
        );
    }

    fn assert_admin(&self) {
        assert_eq!(
            self.admin,
            msg::source(),
            "Only admin can send that message"
        );
    }
}

static mut MT_LOGIC: Option<MTLogic> = None;

pub enum TransactionStatus {
    InProgress,
    Success,
    Failure,
}

#[no_mangle]
unsafe extern "C" fn init() {
    let init_config: InitMTLogic = msg::load().expect("Unable to decode `InitMTLogic`");
    let mt_logic = MTLogic {
        admin: init_config.admin,
        storage_code_hash: init_config.storage_code_hash,
        mtoken_id: msg::source(),
        ..Default::default()
    };
    MT_LOGIC = Some(mt_logic);
}

#[no_mangle]
unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let query: MTLogicState = msg::load().expect("Unable to decode `MTLogicState`");
    let logic: &mut MTLogic = MT_LOGIC.get_or_insert(Default::default());

    let encoded = match query {
        MTLogicState::Storages => {
            let storages = Vec::from_iter(logic.id_to_storage.clone().into_iter());
            MTLogicStateReply::Storages(storages)
        }
    }
    .encode();

    gstd::util::to_leak_ptr(encoded)
}

#[gstd::async_main]
async fn main() {
    let action: MTLogicAction = msg::load().expect("Error in loading `MTLogicAction`");
    let logic: &mut MTLogic = unsafe { MT_LOGIC.get_or_insert(Default::default()) };

    match action {
        MTLogicAction::Message {
            transaction_hash,
            account,
            payload,
        } => logic.message(transaction_hash, &account, &payload).await,
        MTLogicAction::GetBalance { token_id, account } => {
            logic.get_balance(token_id, &account).await
        }
        MTLogicAction::GetApproval {
            account,
            approval_target,
        } => logic.get_approval(&account, &approval_target).await,
        MTLogicAction::UpdateStorageCodeHash(storage_code_hash) => {
            logic.update_storage_hash(storage_code_hash)
        }
        MTLogicAction::Clear(transaction_hash) => logic.clear(transaction_hash),
        _ => {}
    }
}

fn reply_err() {
    msg::reply(MTLogicEvent::Err, 0).expect("Error in sending a reply `MTLogicEvent::Err`");
}

fn reply_ok() {
    msg::reply(MTLogicEvent::Ok, 0).expect("Error in sending a reply `MTLogicEvent::Ok`");
}

fn send_delayed_clear(transaction_hash: H256) {
    msg::send_delayed(
        exec::program_id(),
        MTLogicAction::Clear(transaction_hash),
        0,
        DELAY,
    )
    .expect("Error in sending a delayled message `MTStorageAction::Clear`");
}
