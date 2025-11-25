use ark_bn254::Fr;
use ark_ff::PrimeField;
use ethers_core::types::Address;
use ethers_core::rand::thread_rng;
use ethers_core::k256::ecdsa::SigningKey;
use ethers_core::k256::SecretKey;

#[derive(Clone, Debug)]
pub struct Account {
    pub secret: SecretKey,
    pub address: Address,
}

impl Account {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let secret = SecretKey::random(&mut rng);
        let signing_key = SigningKey::from(&secret);
        let address = ethers_core::utils::secret_key_to_address(&signing_key);

        Self {
            secret,
            address,
        }
    }

    pub fn to_field_element(&self) -> Fr {
        // Convert address (20 bytes) to Field Element
        let bytes = self.address.as_bytes();
        Fr::from_be_bytes_mod_order(bytes)
    }
}
