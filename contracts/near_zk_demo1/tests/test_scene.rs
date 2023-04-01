mod common;

use crate::common::*;

#[tokio::test]
async fn test_sence() -> anyhow::Result<()> {
    let worker = workspaces::sandbox().await?;
    let (root, deploy_account, near_zk_demo1) = initialize_contracts_and_users(&worker, "deploy", 3).await?;

    let alice = create_account(&worker, &root, "alice", None).await;
    let bob = create_account(&worker, &root, "bob", None).await;
    let carol = create_account(&worker, &root, "carol", None).await;
    
    println!("current owner: {:?}", view_get_owner(&worker, &near_zk_demo1, &deploy_account).await?);

    println!("");
    println!(" ====== add_security_question ====== ");

    let alice_question = U256::from_big_endian("What's your favorite food".as_bytes());
    let alice_answer = U256::from_big_endian("ice cream".as_bytes());
    let alice_leaf = hash2(alice_question.clone(), alice_answer);
    println!("alice_question: {:?}", alice_question.to_string());
    println!("alice_answer: {:?}", alice_answer.to_string());
    println!("alice_leaf: {:?}", alice_leaf.to_string());
    call_add_security_question(&worker, &near_zk_demo1, &deploy_account, alice_question.to_string(), alice_leaf.to_string()).await?.is_success();
    
    let bob_question = U256::from_big_endian("where are you from".as_bytes());
    let bob_answer = U256::from_big_endian("china".as_bytes());
    let bob_leaf = hash2(bob_question.clone(), bob_answer);
    println!("bob_question: {:?}", bob_question.to_string());
    println!("bob_answer: {:?}", bob_answer.to_string());
    println!("bob_leaf: {:?}", bob_leaf.to_string());
    call_add_security_question(&worker, &near_zk_demo1, &deploy_account, bob_question.to_string(), bob_leaf.to_string()).await?.is_success();
    
    let carol_question = U256::from_big_endian("What's your favorite number".as_bytes());
    let carol_answer = U256::from_big_endian("666".as_bytes());
    let carol_leaf = hash2(carol_question.clone(), carol_answer);
    println!("carol_question: {:?}", carol_question.to_string());
    println!("carol_answer: {:?}", carol_answer.to_string());
    println!("carol_leaf: {:?}", carol_leaf.to_string());
    call_add_security_question(&worker, &near_zk_demo1, &deploy_account, carol_question.to_string(), carol_leaf.to_string()).await?.is_success();

    let questions = view_get_questions(&worker, &near_zk_demo1, &deploy_account).await?;
    println!("current questions: {:?}", question_to_utf8(questions));

    println!("");
    println!(" ====== update_security_question ====== ");

    let new_alice_question = U256::from_big_endian("which season do you like".as_bytes());
    let new_alice_answer = U256::from_big_endian("autumn".as_bytes());
    let new_alice_leaf = hash2(new_alice_question.clone(), new_alice_answer);
    println!("new_alice_question: {:?}", new_alice_question.to_string());
    println!("new_alice_answer: {:?}", new_alice_answer.to_string());
    println!("new_alice_leaf: {:?}", new_alice_leaf.to_string());
    
    println!("");
    println!("generate circom update proof...");
    println!("");

    let update_proof = std::fs::read_to_string("./tests/data/update_proof.json")
        .expect("Invalid update proof file path");
    let update_public = std::fs::read_to_string("./tests/data/update_public.json")
        .expect("Invalid update public input file path");

    call_update_security_question(&worker, &near_zk_demo1, &alice, update_proof, update_public).await?;

    let questions = view_get_questions(&worker, &near_zk_demo1, &deploy_account).await?;
    println!("current questions: {:?}", question_to_utf8(questions));

    println!("");
    println!("recover...");
    println!("");

    
    println!("alice proof_path: {:?}", view_get_proof_path(&worker, &near_zk_demo1, &alice, new_alice_question.to_string()).await?);
    let recover_proof0 = std::fs::read_to_string("./tests/data/recover_proof0.json")
        .expect("Invalid recover proof file path");
    let recover_public0 = std::fs::read_to_string("./tests/data/recover_public0.json")
        .expect("Invalid recover public input file path");
    println!("current recovers:{:?}", view_get_recovers(&worker, &near_zk_demo1, &deploy_account).await?);
    call_recover(&worker, &near_zk_demo1, &alice, recover_proof0, recover_public0).await?;
    println!("current recovers:{:?}", view_get_recovers(&worker, &near_zk_demo1, &deploy_account).await?);

    println!("bob proof_path: {:?}", view_get_proof_path(&worker, &near_zk_demo1, &alice, bob_question.to_string()).await?);
    let recover_proof1 = std::fs::read_to_string("./tests/data/recover_proof1.json")
        .expect("Invalid recover proof file path");
    let recover_public1 = std::fs::read_to_string("./tests/data/recover_public1.json")
        .expect("Invalid recover public input file path");
    
    call_recover(&worker, &near_zk_demo1, &bob, recover_proof1, recover_public1).await?;
    println!("current recovers:{:?}", view_get_recovers(&worker, &near_zk_demo1, &deploy_account).await?);
    
    println!("carol proof_path: {:?}", view_get_proof_path(&worker, &near_zk_demo1, &alice, carol_question.to_string()).await?);
    let recover_proof2 = std::fs::read_to_string("./tests/data/recover_proof2.json")
        .expect("Invalid recover proof file path");
    let recover_public2 = std::fs::read_to_string("./tests/data/recover_public2.json")
        .expect("Invalid recover public input file path");

    call_recover(&worker, &near_zk_demo1, &carol, recover_proof2, recover_public2).await?;
    println!("current recovers:{:?}", view_get_recovers(&worker, &near_zk_demo1, &deploy_account).await?);
    
    println!("owner: {:?}", view_get_owner(&worker, &near_zk_demo1, &deploy_account).await?);

    Ok(())
}

fn question_to_utf8(questions: Vec<String>) -> Vec<String> {
    questions.into_iter().map(|v| {
        let mut bytes = [0u8; 32];
        let v_u256 = U256::from_str_radix(&v, 10).unwrap();
        v_u256.to_big_endian(&mut bytes);
        let res = String::from_utf8(bytes.to_vec()).unwrap();
        res.replace("\0", "")
    }).collect::<Vec<String>>()
}