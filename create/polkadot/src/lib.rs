#![cfg_attr(not(feature = "std"), no_std)]

use std::str::FromStr;

use anyhow::{anyhow, bail, Result};
use codec::{Compact, Decode, Encode};
use hex::FromHex;
use mainnet_runtime::{SignedExtra, UncheckedExtrinsic};
use serde::{Deserialize, Serialize};
use sp_core::{Pair, ed25519};
use sp_core::crypto::Ss58AddressFormat;
use sp_runtime::AnySignature;
use sp_runtime::generic::Era;

pub use extrinsic::*;
pub use extrinsic_params::*;

pub mod extrinsic;
pub mod compose;
pub mod extrinsic_params;
pub mod keys;
pub mod rpc;
pub mod client;


pub fn signed_extrinsic(hash: String, seed: String, to: String, amount: u128, nonce: u32, spec_version: u32, transaction_version: u32, network_id: u16, module_index: u8, call_index: u8) -> Result<String> {
    // println!("{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}",hash,seed,to,amount,nonce,spec_version,transaction_version,network_id);

    let pair = ed25519::Pair::from_string(seed.as_str(), None).map_err(|e| anyhow!("gen pair error {:?}",e))?;
    sp_core::crypto::set_default_ss58_version(Ss58AddressFormat::custom(network_id));
    let to_addr = AccountId::from_str(to.as_str()).map_err(|e| anyhow!("gen to addr error {:?}",e))?;
    let address = GenericAddress::Id(to_addr);

    let genesis_hash = sp_core::H256::from_str(hash.as_str()).map_err(|e| anyhow!("parse hash error {:?}",e))?;

    let call = ([module_index, call_index], address, Compact(amount));

    let tx_params = PlainTipExtrinsicParamsBuilder::default();

    // other
    // let tx_params = PlainTipExtrinsicParamsBuilder::new()
    //     .era(Era::mortal(period, h.number.into()), head)
    //     .tip(0);
    let extrinsic_params = PlainTipExtrinsicParams::new(
        spec_version,
        transaction_version,
        nonce,
        genesis_hash,
        tx_params,
    );
    let xt = compose_extrinsic_offline!(
            pair,
            call,
            extrinsic_params
        );
    Ok(format!("{:?}", xt.hex_encode()))
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ext() {
        //13JZbUWwxLZ4pLqLq92UmvkFVEXK8FLqR9ETNVvGUfdjyFxz
        let hash = "0x0221153ac1bfb2f2dd3e7f753b02962ae4bd6822d836db65ad65acd4579b7b92".to_string();
        let seed = "0x6b9e9bcc6c7f5a2a351bb81a848855899f4e481da3cca78046fefbed6bd11789".to_string();
        let to = "14E5nqKAp3oAJcmzgZhUD2RcptBeUBScxKHgJKU4HPNcKVf3".to_string();

        let result = signed_extrinsic(hash, seed, to, 100,0,9360,19,0,5,0).unwrap();
        println!("{:?}", result);

    }

    #[test]
    fn test_decode_extrinsic() {
        // let raw = "c1018400fadda0af24a7e6d0bba8d3dd0615915b654d68e8735e213fb37577fc681b3d27010c788166b53ce53f0ec566c950d098b47fc7545ab8b9479dd10f48c84222cd722d5e4d2b3463f1995a336cd0684e2bd16c7979641c5c4a54913de9d8826bf98f4703040009130b0030ef7dba02".to_string();
        // let res = decode_extrinsic(raw).unwrap();
        // println!("{:?}", res);
    }
}
