use std::marker::PhantomData;
use anyhow::Result;
use codec::Encode;

pub struct StorageKey(
    pub Vec<u8>
);

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct StorageValue {
    module_prefix: Vec<u8>,
    storage_prefix: Vec<u8>,
}


impl StorageValue {
    pub fn key(&self) -> StorageKey {
        let mut bytes = sp_core::twox_128(&self.module_prefix).to_vec();
        bytes.extend(&sp_core::twox_128(&self.storage_prefix)[..]);
        StorageKey(bytes)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct StorageMap<K> {
    _marker: PhantomData<K>,
    module_prefix: Vec<u8>,
    storage_prefix: Vec<u8>,
    hasher: StorageHasher,
}

impl <K:Encode>StorageMap<K> {
    pub fn new(module_prefix: &str, storage_prefix: &str, hash: StorageHasher) -> Self {
        StorageMap {
            _marker: Default::default(),
            module_prefix: module_prefix.as_bytes().to_vec(),
            storage_prefix: storage_prefix.as_bytes().to_vec(),
            hasher: hash,
        }
    }
    pub fn key(&self,key:K)->StorageKey{
        let mut bytes = sp_core::twox_128(&self.module_prefix).to_vec();
        bytes.extend(&sp_core::twox_128(&self.storage_prefix)[..]);
        bytes.extend(key_hash(&key, &self.hasher));
        StorageKey(bytes)
    }


}

#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum StorageHasher {
    /// 128-bit Blake2 hash.
    Blake2_128,
    /// 256-bit Blake2 hash.
    Blake2_256,
    /// Multiple 128-bit Blake2 hashes concatenated.
    Blake2_128Concat,
    /// 128-bit XX hash.
    Twox128,
    /// 256-bit XX hash.
    Twox256,
    /// Multiple 64-bit XX hashes concatenated.
    Twox64Concat,
    /// Identity hashing (no hashing).
    Identity,
}

fn key_hash<K: Encode>(key: &K, hasher: &StorageHasher) -> Vec<u8> {
    let encoded_key = key.encode();
    match hasher {
        StorageHasher::Blake2_128 => {
            sp_core::blake2_128(&encoded_key).to_vec()
        }
        StorageHasher::Blake2_256 => {
            sp_core::blake2_256(&encoded_key).to_vec()
        }
        StorageHasher::Blake2_128Concat => {
            let x: &[u8] = encoded_key.as_slice();
            sp_core::blake2_128(x).iter().chain(x.iter()).cloned().collect::<Vec<_>>()
        }
        StorageHasher::Twox128 => {
            sp_core::twox_128(&encoded_key).to_vec()
        }
        StorageHasher::Twox256 => {
            sp_core::twox_256(&encoded_key).to_vec()
        }
        StorageHasher::Twox64Concat => {
            sp_core::twox_64(&encoded_key).iter().chain(&encoded_key).cloned().collect()
        }
        StorageHasher::Identity => {
            encoded_key.to_vec()
        }
    }
}


pub fn blake2_128_concat(x: &[u8]) -> Result<String> {
    let vec = sp_core::blake2_128(x).iter().chain(x.iter()).cloned().collect::<Vec<_>>();
    let res = hex::encode(&vec);
    Ok(res)
}


pub fn twox_64_concat(x: &[u8]) -> Result<String> {
    let vec = sp_core::twox_64(x)
        .iter()
        .chain(x)
        .cloned()
        .collect::<Vec<_>>();
    let res = hex::encode(&vec);
    Ok(res)
}


#[cfg(test)]
mod test {
    #[test]
    fn test() {
        //12MJRZNE1xvgbnMAQpnRYFKJLjNH8HvTMKxGeVe6P13i1fSx
    }
}
