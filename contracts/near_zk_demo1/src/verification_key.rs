
use crate::*;

#[derive(Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct CircomVerificationKey {
    pub protocol: String,
    pub curve: String,
    #[serde(rename = "nPublic")]
    pub n_public: u64,
    pub vk_alpha_1: CircomG1Affine,
    pub vk_beta_2: CircomG2Affine,
    pub vk_gamma_2: CircomG2Affine,
    pub vk_delta_2: CircomG2Affine,
    pub vk_alphabeta_12: Vec<Vec<Vec<String>>>,
    #[serde(rename = "IC")]
    pub ic: Vec<CircomG1Affine>,
}


// impl From<CircomVerificationKey> for VerifyingKey<Bls12_381> {
//     fn from(src: CircomVerificationKey) -> Self {
//         let gamma_abc_g1: Vec<G1Affine> =
//             src.ic.iter().map(|v| g1_affine(v)).collect();
//         VerifyingKey {
//             alpha_g1: g1_affine(&src.vk_alpha_1),
//             beta_g2: g2_affine(&src.vk_beta_2),
//             gamma_g2: g2_affine(&src.vk_gamma_2),
//             delta_g2: g2_affine(&src.vk_delta_2),
//             gamma_abc_g1,
//         }
//     }
// }

impl From<CircomVerificationKey> for VerifyingKey<Bn254> {
    fn from(src: CircomVerificationKey) -> Self {
        let gamma_abc_g1: Vec<G1Affine> =
            src.ic.iter().map(|v| g1_affine(v)).collect();
        VerifyingKey {
            alpha_g1: g1_affine(&src.vk_alpha_1),
            beta_g2: g2_affine(&src.vk_beta_2),
            gamma_g2: g2_affine(&src.vk_gamma_2),
            delta_g2: g2_affine(&src.vk_delta_2),
            gamma_abc_g1,
        }
    }
}