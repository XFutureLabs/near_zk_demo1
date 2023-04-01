use crate::*;

pub async fn view_get_questions (
    worker: &Worker<Sandbox>,
    near_zk_demo1: &Contract,
    user: &Account,
) -> anyhow::Result<Vec<String>>{
    user.call(worker, near_zk_demo1.id(), "get_questions")
        .view()
        .await?.json::<Vec<String>>()
}

pub async fn view_get_recovers (
    worker: &Worker<Sandbox>,
    near_zk_demo1: &Contract,
    user: &Account,
) -> anyhow::Result<Vec<String>>{
    user.call(worker, near_zk_demo1.id(), "get_recovers")
        .view()
        .await?.json::<Vec<String>>()
}

pub async fn view_get_owner (
    worker: &Worker<Sandbox>,
    near_zk_demo1: &Contract,
    user: &Account,
) -> anyhow::Result<AccountId>{
    user.call(worker, near_zk_demo1.id(), "get_owner")
        .view()
        .await?.json::<AccountId>()
}

pub async fn view_get_proof_path (
    worker: &Worker<Sandbox>,
    near_zk_demo1: &Contract,
    user: &Account,
    question: String
) -> anyhow::Result<Option<(String, Vec<String>, Vec<String>)>>{
    user.call(worker, near_zk_demo1.id(), "get_proof_path")
        .args_json(json!({
            "question": question
        }))?
        .view()
        .await?.json::<Option<(String, Vec<String>, Vec<String>)>>()
}

pub async fn call_add_security_question (
    worker: &Worker<Sandbox>,
    near_zk_demo1: &Contract,
    user: &Account,
    question: String,
    leaf: String,
) -> anyhow::Result<CallExecutionDetails>{
    user.call(worker, near_zk_demo1.id(), "add_security_question")
        .args_json(json!({
            "question": question,
            "leaf": leaf,
        }))?
        .max_gas()
        .transact()
        .await
}

pub async fn call_update_security_question (
    worker: &Worker<Sandbox>,
    near_zk_demo1: &Contract,
    user: &Account,
    proof_string: String,
    public_input_string: String,
) -> anyhow::Result<CallExecutionDetails>{
    user.call(worker, near_zk_demo1.id(), "update_security_question")
        .args_json(json!({
            "proof_string": proof_string,
            "public_input_string": public_input_string,
        }))?
        .max_gas()
        .transact()
        .await
}

pub async fn call_recover (
    worker: &Worker<Sandbox>,
    near_zk_demo1: &Contract,
    user: &Account,
    proof_string: String,
    public_input_string: String,
) -> anyhow::Result<CallExecutionDetails>{
    user.call(worker, near_zk_demo1.id(), "recover")
        .args_json(json!({
            "proof_string": proof_string,
            "public_input_string": public_input_string,
        }))?
        .max_gas()
        .transact()
        .await
}
