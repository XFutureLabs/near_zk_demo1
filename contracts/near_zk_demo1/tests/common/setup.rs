use crate::*;

pub async fn initialize_contracts_and_users(
    worker: &Worker<Sandbox>,
    account_id: &str,
    depth: usize
) -> anyhow::Result<(Account, Account, Contract)> {
    let root = worker.root_account();

    let deploy_account = root
        .create_subaccount(worker, account_id)
        .initial_balance(parse_near!("50 N"))
        .transact()
        .await?
        .unwrap();

    let near_zk_demo1_contract = deploy_account
        .deploy(&worker, &std::fs::read(format!("../../res/near_zk_demo1.wasm"))?)
        .await?
        .unwrap();

    let update_verification_key_string = std::fs::read_to_string("../../circuits/out/update_verification_key.json")
        .expect("Invalid withdraw verification key file path");
    let recover_verification_key_string = std::fs::read_to_string("../../circuits/out/recover_verification_key.json")
        .expect("Invalid split verification key file path");

    near_zk_demo1_contract.call(worker, "new")
        .args_json(json!({
            "depth": depth,
            "update_verification_key": update_verification_key_string,
            "recover_verification_key": recover_verification_key_string, 
        }))?
        .gas(300_000_000_000_000)
        .transact()
        .await?;
    Ok((root, deploy_account, near_zk_demo1_contract))
}

pub async fn create_account(
    worker: &Worker<Sandbox>,
    master: &Account,
    account_id: &str,
    balance: Option<u128>,
) -> Account {
    let balance = if let Some(balance) = balance {
        balance
    } else {
        parse_near!("50 N")
    };
    master
        .create_subaccount(&worker, account_id)
        .initial_balance(balance)
        .transact()
        .await
        .unwrap()
        .unwrap()
}