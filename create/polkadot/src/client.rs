use anyhow::{anyhow, Result};
use codec::{Decode, Encode};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use sp_core::crypto::{Ss58AddressFormat, Ss58Codec};
use sp_core::ed25519;
use uuid::Uuid;

use crate::rpc::storage::{*};
use crate::rpc::types::{*};

use super::signed_extrinsic;

pub struct Client {
    endpoint: String,
    client: reqwest::Client,
}


impl Client {
    pub fn new(endpoint: String) -> Self {
        let client = reqwest::Client::new();
        Client {
            endpoint,
            client,
        }
    }

    pub async fn runtime_version(&self) -> Result<RuntimeVersion> {
        let result: String = self.request_no_type::<Vec<String>>("state_getRuntimeVersion".to_string(), Vec::new()).await?;
        let response: JsonRpcResp<RuntimeVersion> = serde_json::from_str(&result)?;
        Ok(response.result)
    }


    pub async fn transfer(&self, seed: String, from: String, to: String, amount: u128) -> Result<Vec<u8>> {
        let genesis_hash = self.genesis_hash().await?;
        let runtime_version = self.runtime_version().await?;
        let account_data = self.system_account(&from).await?;
        let account_info:AccountInfo = Decode::decode(& mut account_data.as_slice())?;
        let tx_data = signed_extrinsic(genesis_hash, seed, to, amount, account_info.nonce, runtime_version.spec_version, runtime_version.transaction_version, 0 as u16, 5 as u8, 0 as u8)?;
        let data = tx_data.replace("\"", "").strip_prefix("0x").unwrap().to_string();
        let tx_hash = self.author_submit_extrinsic(data).await?;
        Ok(tx_hash)
    }


    pub async fn storage_map_key(&self, module_prefix: &str, storage_prefix: &str, addr: &str) -> Result<String> {
        let public = ed25519::Public::from_ss58check(addr)?;
        let storage_map = StorageMap::new(module_prefix, storage_prefix, StorageHasher::Blake2_128Concat);
        let storage_key = storage_map.key(public);
        let key = hex::encode(storage_key.0);
        Ok(key)
    }



    pub async fn author_submit_extrinsic(&self, data: String) -> Result<Vec<u8>> {
        let mut params = Vec::<String>::new();
        params.push(data);
        let result: String = self.request_no_type::<Vec<String>>("author_submitExtrinsic".to_string(), params).await?;
        let response: JsonRpcResp<String> = serde_json::from_str(&result)?;
        Ok(response.result.into_bytes())
    }


    pub async fn system_account(&self, addr: &str) -> Result<Vec<u8>> {
        let key = self.storage_map_key("System", "Account", addr).await?;
        let mut params = Vec::<String>::new();
        params.push(key);
        let account = self.http_post::<Vec<String>>("state_getStorage".to_string(), params).await?;
        Ok(account)
    }


    pub async fn finalize_head(&self) -> Result<String> {
        let result: String = self.request_no_type::<Vec<String>>("chain_getFinalizedHead".to_string(), Vec::new()).await?;
        let response: JsonRpcResp<String> = serde_json::from_str(&result)?;
        Ok(response.result)
    }

    pub async fn genesis_hash(&self) -> Result<String> {
        let mut params = Vec::<u64>::new();
        params.push(0);
        let result: String = self.request_no_type::<Vec<u64>>("chain_getBlockHash".to_string(), params).await?;
        let response: JsonRpcResp<String> = serde_json::from_str(&result)?;
        Ok(response.result)
    }


    pub async fn request_no_type<T: Serialize>(&self, method: String, params: T) -> Result<String> {
        let req: JsonRpcReq<T> = JsonRpcReq::new(method, params);
        let body = serde_json::to_string(&req)?;
        let res = self.client.post(&self.endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send().await?.text().await?;
        info!("result: {}",res);
        Ok(res)
    }

    pub async fn http_post<T: Serialize>(&self, method: String, params: T) -> Result<Vec<u8>> {
        let req: JsonRpcReq<T> = JsonRpcReq::new(method, params);
        let body = serde_json::to_string(&req)?;
        let result = self.client.post(&self.endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send().await?.text().await?;
        let response: JsonRpcResp<String> = serde_json::from_str(&result)?;
        debug!("result: {}",response.result);
        let storage = hex::decode(response.result.strip_prefix("0x").unwrap()).unwrap();
        Ok(storage)
    }

    pub async fn post<T: Serialize, R: Decode>(&self, method: String, params: T) -> Result<R> {
        let req: JsonRpcReq<T> = JsonRpcReq::new(method, params);
        let body = serde_json::to_string(&req)?;
        let result = self.client.post(&self.endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send().await?.text().await?;
        let response: JsonRpcResp<String> = serde_json::from_str(&result)?;
        debug!("result: {}",response.result);
        let storage = hex::decode(response.result.strip_prefix("0x").unwrap()).unwrap();
        let result: R = Decode::decode(&mut storage.as_slice()).unwrap();
        Ok(result)
    }
}


#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct JsonRpcReq<T> {
    id: u64,
    #[serde(rename = "jsonrpc")]
    json_rpc: String,
    method: String,
    params: T,
}


impl<T> JsonRpcReq<T> {
    fn new(method: String, params: T) -> Self {
        let id = Uuid::new_v4();
        JsonRpcReq {
            id: id.to_u128_le() as u64,
            json_rpc: "2.0".to_string(),
            method,
            params,
        }
    }
}


#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct JsonRpcResp<T> {
    id: u64,
    #[serde(rename = "jsonrpc")]
    json_rpc: String,
    result: T,

}

//const ENDPOINT: &str = "https://rpc.polkadot.io";
const ENDPOINT: &str = "http://127.0.0.1:9933";

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::f32::consts::E;
    use std::future::Future;

    use super::*;

    #[test]
    fn test001() {
        let client = Client::new("https://rpc.polkadot.io".to_string());
        futures::executor::block_on(async {
            match client.system_account("15QFBQY6TF6Abr6vA1r6opRh6RbRSMWgBC1PcCMDDzRSEXfn").await {
                Ok(account) => {
                    // println!("{}", account);
                }
                Err(e) => {
                    println!("ddd {}", e.to_string());
                }
            }
        });
    }


    #[test]
    fn test03() {
        let req = JsonRpcReq::<Vec<String>>::new("dadaf".to_string(), Vec::new());
        println!("{:?}", req);
        let resp = serde_json::to_string(&req).unwrap();
        println!("{:?}", resp);
    }
}