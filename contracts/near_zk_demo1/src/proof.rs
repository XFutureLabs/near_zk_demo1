use crate::*;


#[derive(Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct CircomProof {
    pub pi_a: CircomG1Affine,
    pub pi_b: CircomG2Affine,
    pub pi_c: CircomG1Affine,
    pub protocol: String,
    pub curve: String,
}

// impl From<CircomProof> for Proof<Bls12_381> {
//     fn from(src: CircomProof) -> Self {
//         Proof {
//             a: g1_affine(&src.pi_a),
//             b: g2_affine(&src.pi_b),
//             c: g1_affine(&src.pi_c),
//         }
//     }
// }

impl From<CircomProof> for Proof<Bn254> {
    fn from(src: CircomProof) -> Self {
        Proof {
            a: g1_affine(&src.pi_a),
            b: g2_affine(&src.pi_b),
            c: g1_affine(&src.pi_c),
        }
    }
}