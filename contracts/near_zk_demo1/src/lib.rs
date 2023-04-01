use std::{
    fmt::Debug,
    str::FromStr,
    iter::{once, repeat, successors},
};

use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    near_bindgen, PanicOnDefault, require, AccountId, env
};
use near_sdk::serde::{Serialize, Deserialize};

use ark_groth16::{VerifyingKey, Proof};
use ark_bn254::{Fr, Fq, Fq2, G1Affine, G2Affine, G1Projective, G2Projective, Bn254};
use ark_ff::biginteger::BigInteger256;

mod proof;
mod verification_key;

mod poseidon;
mod merkle_tree;
mod utils;


pub use proof::*;
pub use verification_key::*;

pub use poseidon::*;
pub use merkle_tree::*;
pub use utils::*;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub update_verification_key: String,
    pub recover_verification_key: String,
    pub owner_id: AccountId,
    pub tree: MerkleTree<PoseidonHash>,
    pub questions: Vec<String>,
    pub recovers: Vec<String>,
    pub new_owner: Option<AccountId>
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(depth: usize, update_verification_key: String, recover_verification_key: String) -> Self {
        Self {
            update_verification_key,
            recover_verification_key,
            owner_id: env::predecessor_account_id(),
            tree: MerkleTree::new(depth, U256::zero()),
            questions: Vec::new(),
            recovers: Vec::new(),
            new_owner: None
        }
    }


    pub fn add_security_question(&mut self, question: String, leaf: String) {
        assert!(self.owner_id == env::predecessor_account_id(), "Not onwer");
        assert!(self.tree.num_leaves() > self.questions.len(), "Questions exceeds upper limit");
        self.tree.set(self.questions.len(), U256::from_str_radix(leaf.as_str(), 10).unwrap());
        self.questions.push(question);
    }

    pub fn update_security_question(&mut self, proof_string: String, public_input_string: String) {
        assert!(self.recovers.is_empty(), "In recover");

        let public_input_vec: Vec<String> = serde_json_wasm::from_str(&public_input_string).expect("Invalid public input");
        let new_leaf = public_input_vec[0].clone();
        let old_root = public_input_vec[1].clone();
        let old_question = public_input_vec[2].clone();
        let new_question = public_input_vec[3].clone();

        assert!(self.tree.root().to_string() == old_root, "Invalid proof: old root");
        self.verify(proof_string, public_input_string, "update".to_string());
        let index = self.questions.iter().position(|v| v == &old_question).expect("Invalid proof: old question");
        self.tree.set(index, U256::from_str_radix(new_leaf.as_str(), 10).unwrap());
        self.questions[index] = new_question;
    }

    pub fn recover(&mut self, proof_string: String, public_input_string: String) {
        let public_input_vec: Vec<String> = serde_json_wasm::from_str(&public_input_string).expect("Invalid public input");
        let nullifier = public_input_vec[0].clone();
        let root = public_input_vec[1].clone();
        let new_owner_string = public_input_vec[2].clone();

        let mut new_owner_bytes = [0u8; 32];
        let new_owner_u256 = U256::from_str_radix(&new_owner_string, 10).unwrap();
        new_owner_u256.to_big_endian(&mut new_owner_bytes);
        let new_owner_utf8 = String::from_utf8(new_owner_bytes.to_vec()).unwrap().replace("\0", "");
        let new_owner_account: AccountId = new_owner_utf8.parse().unwrap();

        if let Some(new_owner) = &self.new_owner {
            assert!(new_owner == &new_owner_account, "Invalid new_owner_account");
        } else {
            self.new_owner = Some(new_owner_account);
        }
        assert!(!self.recovers.contains(&nullifier), "Repeat recover");
        assert!(self.tree.root() == U256::from_str_radix(&root, 10).unwrap(), "Invalid proof: root");

        self.verify(proof_string, public_input_string, "recover".to_string());
        self.recovers.push(nullifier);

        if self.questions.len() == self.recovers.len() {
            let new_owner = self.new_owner.take().unwrap();
            self.owner_id = new_owner;
            let depth = self.tree.depth();
            self.tree = MerkleTree::new(depth, U256::zero());
            self.questions.clear();
            self.recovers.clear();
        }
    }

    pub fn verify(&self, proof_string: String, public_input_string: String, proof_type: String){
        let verification_key = match proof_type.as_str() {
            "update" => self.update_verification_key.clone(),
            "recover" => self.recover_verification_key.clone(),
            _ => panic!("proof_type error")
        };
        let circom_verification_key: CircomVerificationKey = serde_json_wasm::from_str(&verification_key).expect("Invalid verification key");
        assert_eq!(circom_verification_key.protocol, "groth16");
        let verification_key: VerifyingKey<Bn254> = circom_verification_key.into();
        let prepare_verifying_key = ark_groth16::prepare_verifying_key(&verification_key);

        let circom_proof: CircomProof = serde_json_wasm::from_str(&proof_string).expect("Invalid proof");
        assert_eq!(circom_proof.protocol, "groth16");
        let proof: Proof<Bn254> = circom_proof.into();

        let circom_public_input: Vec<String> = serde_json_wasm::from_str(&public_input_string).expect("Invalid public input");
        let pub_inputs: Vec<Fr> = circom_public_input.into_iter().map(|v| Fr::from_str(&v).unwrap()).collect();
        require!(ark_groth16::verify_proof(&prepare_verifying_key, &proof, &pub_inputs[..]).unwrap());
    }

    pub fn get_proof_path(&self, question: String) -> Option<(String, Vec<String>, Vec<String>)> {
        let mut path_indices = vec![];
        let mut siblings = vec![];
        if let Some(index) = self.questions.iter().position(|v| v == &question) {
            if let Some(path) = self.tree.proof(index) {
                for item in path.0.iter() {
                    match item {
                        Branch::Left(v) => {
                            path_indices.push("0".to_string());
                            siblings.push(v.to_string());
                        },
                        Branch::Right(v) => {
                            path_indices.push("1".to_string());
                            siblings.push(v.to_string());
                        }
                    }
                }
                Some((self.tree.root().to_string(), path_indices, siblings))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn get_questions(&self) -> Vec<String> {
        self.questions.clone()
    }

    pub fn get_recovers(&self) -> Vec<String> {
        self.recovers.clone()
    }

    pub fn get_owner(&self) -> AccountId {
        self.owner_id.clone()
    }
}
