mod utils_gclient;

use gstd::ActorId;
use mt_logic_io::TokenId;
use std::mem;
use utils_gclient::*;

#[tokio::test]
pub async fn success_create_ft_gclient() -> gclient::Result<()> {
    let (api, program_id, mut listener) = setup_gclient().await?;

    let mut tx_id = 0;
    let initial_amount = 1000000;

    let api = api.with(USER_ACCOUNTS[0])?;
    let mut listener = api.subscribe().await?;
    let user_account_0 = ActorId::new(api.account_id().clone().into());
    let token_id: TokenId = 1 << (mem::size_of::<TokenId>() * 8 / 2);
    println!("BEFORE_CREATE 1");
    mtoken_create(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        initial_amount,
        String::from("https://example.com"),
        false,
    )
    .await?;
    assert_eq!(
        mtoken_get_balance(&api, &mut listener, &program_id, token_id, user_account_0).await?,
        initial_amount
    );
    tx_id += 1;
    println!("AFTER_CREATE 1");

    let api = api.with(USER_ACCOUNTS[1])?;
    let mut listener = api.subscribe().await?;
    let user_account_1 = ActorId::new(api.account_id().clone().into());
    let token_id: TokenId = 2 << (mem::size_of::<TokenId>() * 8 / 2);
    println!("BEFORE_CREATE 2");
    mtoken_create(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        initial_amount * 2,
        String::from("https://example.com"),
        false,
    )
    .await?;
    println!("BEFORE_BALANCE 2");
    assert_eq!(
        mtoken_get_balance(&api, &mut listener, &program_id, token_id, user_account_1).await?,
        initial_amount * 2
    );
    tx_id += 1;
    println!("AFTER_CREATE 2");

    let api = api.with(USER_ACCOUNTS[0])?;
    let mut listener = api.subscribe().await?;
    let token_id: TokenId = 3 << (mem::size_of::<TokenId>() * 8 / 2);
    println!("BEFORE_CREATE 3");
    mtoken_create(
        &api,
        &mut listener,
        &program_id,
        tx_id,
        initial_amount / 10000,
        String::from("https://example.com"),
        false,
    )
    .await?;
    assert_eq!(
        mtoken_get_balance(&api, &mut listener, &program_id, token_id, user_account_0).await?,
        initial_amount / 10000
    );
    println!("AFTER_CREATE 3");

    Ok(())
}
