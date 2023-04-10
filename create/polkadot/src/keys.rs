use bip39::{Language, Mnemonic, Seed};
pub use bip39::MnemonicType;
use sp_core::{ed25519, Pair};
use sp_core::crypto::Ss58Codec;
use sp_runtime::traits::IdentifyAccount;

use crate::Ss58AddressFormat;

pub struct Key {}


impl Key {
    pub fn generate_phrase(mnem_type: MnemonicType) -> String {
        /// create a new randomly generated mnemonic phrase
        let mnemonic = Mnemonic::new(mnem_type, Language::English);
        /// get the phrase
        let phrase: &str = mnemonic.phrase();
        phrase.to_string()
    }

    pub fn generate_seed(phrase: &str, password: Option<&str>) -> String {
        let (_, seed) = ed25519::Pair::from_phrase(phrase, password).unwrap();
        let seed_hex = hex::encode(&seed);
        seed_hex
    }

    pub fn address_from_phrase(phrase: &str, password: Option<&str>) -> String {
        let seed = Key::generate_seed(phrase, password);
        let address = Key::address(&seed, 0);
        address
    }


    pub fn address(seed: &str, network_id: u16) -> String {
        let result = hex::decode(seed).unwrap();
        let pair: ed25519::Pair = ed25519::Pair::from_seed_slice(&result).unwrap();
        let address = pair.public().into_account().to_ss58check_with_version(Ss58AddressFormat::custom(network_id));
        address
    }

    pub fn sign(phrase: &str, msg: &str, password: Option<&str>) -> String {
        let (pair, _) = ed25519::Pair::from_phrase(phrase, password).unwrap();
        let hex_msg = hex::decode(msg).unwrap();
        let signature = pair.sign(&hex_msg);
        hex::encode(signature)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_key() {
        let phrase = Key::generate_phrase(MnemonicType::Words12);
        let phrase = "palace utility secret spice retire air notice wage shove alcohol render sphere".to_string();
        println!("{:?}", phrase);
        let seed = Key::generate_seed(&phrase, None);
        let seed = format!("{}", seed);
        println!("{:?}", seed);
        let address = Key::address(&seed, 0);
        println!("{:?}", address);
        let signature = Key::sign(&phrase, "b23226e4e2bc629fc5e22ac16d0ee3ae68df31210e0871047f054caeff9d00db", None);
        println!("{:?}", signature);
    }


    #[test]
    fn test() {
        use bip39::{Mnemonic, MnemonicType, Language, Seed};

        /// create a new randomly generated mnemonic phrase
        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);

        /// get the phrase
        let phrase: &str = mnemonic.phrase();
        println!("phrase: {}", phrase);
        let phrase = "palace utility secret spice retire air notice wage shove alcohol render sphere";
        /// get the HD wallet seed
        let seed = Seed::new(&mnemonic, "");

        // get the HD wallet seed as raw bytes
        let seed_bytes: &[u8] = seed.as_bytes();

        // print the HD wallet seed as a hex string
        println!("{:X}", seed);
    }
}