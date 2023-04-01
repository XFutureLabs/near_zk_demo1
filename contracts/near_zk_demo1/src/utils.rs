use crate::*;
use uint::construct_uint;

construct_uint! {
    #[derive(BorshDeserialize, BorshSerialize, Serialize)]
    #[serde(crate = "near_sdk::serde")]
	pub struct U256(4);
}



#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct PoseidonHash;

impl Hasher for PoseidonHash {
    type Hash = U256;

    fn hash_node(left: &Self::Hash, right: &Self::Hash) -> Self::Hash {
        hash2(*left, *right)
    }
}

pub type CircomG1Affine = Vec<String>;
pub type CircomG2Affine = Vec<Vec<String>>;

pub fn g1_affine(g1: &CircomG1Affine) -> G1Affine {
    G1Affine::from(G1Projective::new(
        Fq::from_str(&g1[0]).unwrap(), 
        Fq::from_str(&g1[1]).unwrap(), 
        Fq::from_str(&g1[2]).unwrap()
    ))
}

pub fn g2_affine(g2: &CircomG2Affine) -> G2Affine {
    let x =Fq2::new(
        Fq::from_str(&g2[0][0]).unwrap(), 
        Fq::from_str(&g2[0][1]).unwrap(), 
    );
    let y = Fq2::new(
        Fq::from_str(&g2[1][0]).unwrap(), 
        Fq::from_str(&g2[1][1]).unwrap(), 
    );
    let z = Fq2::new(
        Fq::from_str(&g2[2][0]).unwrap(), 
        Fq::from_str(&g2[2][1]).unwrap(), 
    );
    G2Affine::from(G2Projective::new(x, y, z))
}