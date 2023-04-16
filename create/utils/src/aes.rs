use aes_gcm_siv::{aead::{Aead, generic_array::GenericArray, KeyInit, Payload}, AeadCore, Aes256GcmSiv, Nonce};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::to_vec;

use crate::sha256;

const KEY_LENGTH: usize = 32;

pub fn generate_key(pwd: &[u8]) -> Result<Vec<u8>> {
    let mut key = Vec::<u8>::new();
    if pwd.len() >= KEY_LENGTH {
        for num in 0..KEY_LENGTH {
            key.push(pwd[num]);
        }
    } else {
        while key.len() < KEY_LENGTH {
            for num in 0..pwd.len() {
                if key.len() >= KEY_LENGTH {
                    break;
                }
                key.push(pwd[num])
            }
        }
    }
    Ok(key)
}


pub fn encode(data: &[u8], key: &[u8], nonce: &[u8], salt: &[u8]) -> Result<Vec<u8>> {
    let key = GenericArray::from_slice(key);
    let nonce = GenericArray::from_slice(nonce);
    let aes256gcm_siv = Aes256GcmSiv::new(&key);
    let payload = Payload {
        msg: data,
        aad: salt,
    };
    let result = aes256gcm_siv.encrypt(&nonce, payload).map_err(|e| anyhow!("{}",e))?;
    Ok(result)
}

pub fn decode(data: &[u8], key: &[u8], nonce: &[u8], salt: &[u8]) -> Result<Vec<u8>> {
    let key = GenericArray::from_slice(key);
    let nonce = GenericArray::from_slice(nonce);
    let aes256gcm_siv = Aes256GcmSiv::new(&key);
    let payload = Payload {
        msg: data,
        aad: salt,
    };
    let result = aes256gcm_siv.decrypt(&nonce, payload).map_err(|e| anyhow!("{}",e))?;
    Ok(result)
}


#[derive(Serialize, Deserialize)]
struct Secret {
    nonce: Vec<u8>,
    data: Vec<u8>,
}


pub fn wrapp_encode(data: &[u8], key: &[u8]) -> Result<String> {
    let key = generate_key(key).unwrap();
    let key = GenericArray::from_slice(&key);
    let nonce = Aes256GcmSiv::generate_nonce(rand::thread_rng());
    let salt = sha256(key).unwrap();
    let aes256gcm_siv = Aes256GcmSiv::new(&key);
    let payload = Payload {
        msg: data,
        aad: salt.as_bytes(),
    };
    let result = aes256gcm_siv.encrypt(&nonce, payload).map_err(|e| anyhow!("{}",e))?;
    let secret = Secret { nonce: nonce.to_vec(), data: result };
    let r = serde_json::to_string(&secret).unwrap();
    Ok(r)
}


pub fn wrapp_decode(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let key = generate_key(key).unwrap();
    let secret: Secret = serde_json::from_slice(data).unwrap();
    let nonce = GenericArray::from_slice(&secret.nonce);
    let key = GenericArray::from_slice(&key);

    let salt = sha256(key).unwrap();
    let aes256gcm_siv = Aes256GcmSiv::new(&key);
    let payload = Payload {
        msg: &secret.data,
        aad: salt.as_bytes(),
    };
    let result = aes256gcm_siv.decrypt(&nonce, payload).map_err(|e| anyhow!("{}",e))?;
    Ok(result)
}


#[cfg(test)]
mod test {
    use aes_gcm_siv::aead::AeadMut;
    use aes_gcm_siv::AesGcmSiv;

    use super::*;

    #[test]
    fn test_simple() {
        let key = generate_key("nihao".as_bytes()).unwrap();
        let data = "i love you".as_bytes();
        let encode = wrapp_encode(data, &key).unwrap();
        println!("{}", encode);
        let decode = wrapp_decode(encode.as_bytes(), &key).unwrap();
        println!("{:?}", String::from_utf8(decode))
    }


    #[test]
    fn test_generate_key() {
        let key = generate_key("123456789012345678901234567890aa".as_bytes()).unwrap();
        println!("{:?}", key.len());
        let key = generate_key("123456789012345678901234567890aa123456789012345678901234567890aa".as_bytes()).unwrap();
        println!("{:?}", key.len());
        let key = generate_key("1234567".as_bytes()).unwrap();
        println!("{:?}", key.len());
    }


    #[test]
    fn test01() {
        let nonce = Aes256GcmSiv::generate_nonce(rand::thread_rng());
        println!("{:?}", nonce);
        let key = Aes256GcmSiv::generate_key(rand::thread_rng());
        println!("{:?}", key);
        let msg = "wo he ni";
        let encode_data = encode(msg.as_bytes(), &key, &nonce, &vec![]).unwrap();

        let decode_data = decode(&encode_data, &key, &nonce, &vec![]).unwrap();
        println!("{}", String::from_utf8(decode_data).unwrap());
    }


    #[test]
    fn test() {
        let nonce = Aes256GcmSiv::generate_nonce(rand::thread_rng());
        println!("{:?}", nonce);
        let key = Aes256GcmSiv::generate_key(rand::thread_rng());
        println!("{:?}", key);
        let aes256gcm_siv = Aes256GcmSiv::new(&key);
        let payload = Payload {
            msg: "nihao".as_bytes(),
            aad: "".as_bytes(),
        };
        let data = aes256gcm_siv.encrypt(&nonce, payload).unwrap();
        println!("{:?}", String::from_utf8(data.clone()));

        let decode_payload = Payload {
            msg: data.as_slice(),
            aad: "".as_bytes(),
        };
        let decode_data = aes256gcm_siv.decrypt(&nonce, decode_payload).unwrap();
        println!("{:?}", String::from_utf8(decode_data).unwrap());
    }
}