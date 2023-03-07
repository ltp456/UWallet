use anyhow::Result;
use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use sp_core::crypto::{Ss58AddressFormat, Ss58Codec};
use sp_core::ed25519;
use uuid::Uuid;

use super::storage::{*};
use super::types::{*};

pub struct Client {
    endpoint: String,
    client: reqwest::blocking::Client,
}


impl Client {
    pub fn new(endpoint: String) -> Self {
        let client = reqwest::blocking::Client::new();
        Client {
            endpoint,
            client,
        }
    }

    pub fn storage_map_key(&self, module_prefix: &str, storage_prefix: &str, addr: &str) -> Result<String> {
        let public = ed25519::Public::from_ss58check(addr).unwrap();
        let storage_map = StorageMap::new(module_prefix, storage_prefix, StorageHasher::Blake2_128Concat);
        let storage_key = storage_map.key(public);
        let key = hex::encode(storage_key.0);
        Ok(key)
    }


    pub fn system_account(&self, addr: String) -> Result<AccountInfo> {
        let key = self.storage_map_key("System", "Account", &addr).unwrap();
        let mut params = Vec::<String>::new();
        params.push(key);
        let account = self.post::<Vec<String>, AccountInfo>("state_getStorage".to_string(), params)?;
        Ok(account)
    }


    pub fn finalize_head(&self) -> Result<String> {
        let result: String = self.request_no_type::<Vec<String>>("chain_getFinalizedHead".to_string(), Vec::new())?;
        Ok(result)
    }

    pub fn request_no_type<T: Serialize>(&self,method:String, params: T) -> Result<String> {
        let req: JsonRpcReq<T> = JsonRpcReq::new(method, params);
        let body = serde_json::to_string(&req)?;
        let res = self.client.post(&self.endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()?.text()?;
        let response: JsonRpcResp<String> = serde_json::from_str(&res)?;
        Ok(response.result)
    }


    pub fn post<T: Serialize, R: Decode>(&self, method: String, params: T) -> Result<R> {
        let req: JsonRpcReq<T> = JsonRpcReq::new(method, params);
        let body = serde_json::to_string(&req)?;
        let res = self.client.post(&self.endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .send()?.text()?;
        let response: JsonRpcResp<String> = serde_json::from_str(&res)?;
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


#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_account() {
        let client = Client::new("https://rpc.polkadot.io".to_string());
        let metadata = client.system_account("16mBaA4BPtJzxLchgbHkimRamd4PjnEpELn2N1TS86Hv3NJ7".to_string()).unwrap();
        println!("{:?}", metadata.data.free);
    }


    #[test]
    fn test_client() {
        let client = Client::new("https://rpc.polkadot.io".to_string());
        let metadata = client.finalize_head().unwrap();
        println!("{:?}", metadata);
    }


    #[test]
    fn test01() {
        let resp = reqwest::blocking::get("https://www.baidu.com").unwrap().text().unwrap();
        println!("{:#?}", resp);
    }


    #[test]
    fn test02() {
        let client = reqwest::blocking::Client::new();
        let body = r#"{ "id":100,"jsonrpc":"2.0","method":"state_getMetadata","params":[]}"#;
        let res = client.post("https://rpc.polkadot.io")
            .header("Content-Type", "application/json")
            .body(body)
            .send().unwrap().text().unwrap();
        println!("{:?}", res);
        let resp: JsonRpcResp<String> = serde_json::from_str(&res).unwrap();
        println!("{:?}", resp.result);

        // let body = new_json_req();
        //
        // println!("{:?}",body);
    }


    #[test]
    fn test03() {
        let req = JsonRpcReq::<Vec<String>>::new("dadaf".to_string(), Vec::new());
        println!("{:?}", req);
        let resp = serde_json::to_string(&req).unwrap();
        println!("{:?}", resp);
    }
}