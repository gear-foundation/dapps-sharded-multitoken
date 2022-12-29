#![no_std]

use gstd::{prelude::*, ActorId};
use primitive_types::H256;

pub type TokenId = u128;

#[derive(Debug, Decode, Encode, TypeInfo, Default, Clone, PartialEq, Eq)]
pub struct TokenMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: Option<String>,
    pub reference: Option<String>,
}

#[derive(Encode, Decode, Debug, Copy, Clone, TypeInfo)]
pub enum MTStorageAction {
    GetBalance {
        token_id: TokenId,
        account: ActorId,
    },
    GetApproval {
        account: ActorId,
        approval_target: ActorId,
    },
    GetTokenMetadata(TokenId),
    GetTokenOwner(TokenId),
    GetName,
    GetSymbol,
    GetBaseURI,
    Transfer {
        transaction_hash: H256,
        token_id: TokenId,
        msg_source: ActorId,
        sender: ActorId,
        recipient: ActorId,
        amount: u128,
    },
    Approve {
        transaction_hash: H256,
        msg_source: ActorId,
        account: ActorId,
        approve: bool,
    },
    ClearTransaction(H256),
    IncreaseBalance {
        transaction_hash: H256,
        token_id: TokenId,
        account: ActorId,
        amount: u128,
    },
    DecreaseBalance {
        transaction_hash: H256,
        token_id: TokenId,
        msg_source: ActorId,
        account: ActorId,
        amount: u128,
    },
}

#[derive(Encode, Decode, Clone, Debug, TypeInfo)]
pub enum MTStorageEvent {
    Ok,
    Err,
    Balance(u128),
    Approval(bool),
    TokenMetadata(TokenMetadata),
    TokenOwner(ActorId),
    Name(String),
    Symbol(String),
    BaseURI(String),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum MTStorageState {
    GetBalance {
        token_id: TokenId,
        account: ActorId,
    },
    GetApproval {
        account: ActorId,
        approval_target: ActorId,
    },
    GetTokenMetadata(TokenId),
    GetTokenOwner(TokenId),
    GetName,
    GetSymbol,
    GetBaseURI,
}

#[derive(Encode, Decode, Debug, TypeInfo)]
pub enum MTStorageStateReply {
    Balance(u128),
    Approval(bool),
    TokenMetadata(TokenMetadata),
    TokenOwner(ActorId),
    Name(String),
    Symbol(String),
    BaseURI(String),
}
