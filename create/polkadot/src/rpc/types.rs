use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RuntimeVersion {
    #[serde(rename = "specVersion")]
    pub spec_version: u32,
    #[serde(rename = "authoringVersion")]
    pub authoring_version: u32,
    #[serde(rename = "implName")]
    pub impl_name: String,
    #[serde(rename = "specName")]
    pub spec_name: String,
    #[serde(rename = "stateVersion")]
    pub state_version: u32,
    #[serde(rename = "transactionVersion")]
    pub transaction_version: u32,
}


#[derive(Clone, Eq, PartialEq, Default, Encode, Decode)]
pub struct AccountInfo {
    /// The number of transactions this account has sent.
    pub nonce: u32,
    /// The number of other modules that currently depend on this account's existence. The account
    /// cannot be reaped until this is zero.
    pub consumers: u32,
    /// The number of other modules that allow this account to exist. The account may not be reaped
    /// until this and `sufficients` are both zero.
    pub providers: u32,
    /// The number of modules that allow this account to exist for their own purposes only. The
    /// account may not be reaped until this and `providers` are both zero.
    pub sufficients: u32,
    /// The additional data that belongs to this account. Used to store the balance(s) in a lot of
    /// chains.
    pub data: AccountData,
}


#[derive(Encode, Decode, Clone, PartialEq, Eq, Default)]
pub struct AccountData {
    /// Non-reserved part of the balance. There may still be restrictions on this, but it is the
    /// total pool what may in principle be transferred, reserved and used for tipping.
    ///
    /// This is the only balance that matters in terms of most operations on tokens. It
    /// alone is used to determine the balance when in the contract execution environment.
    pub free: u128,
    /// Balance which is reserved and may not be used at all.
    ///
    /// This can still get slashed, but gets slashed last of all.
    ///
    /// This balance is a 'reserve' balance that other subsystems use in order to set aside tokens
    /// that are still 'owned' by the account holder, but which are suspendable.
    /// This includes named reserve and unnamed reserve.
    pub reserved: u128,
    /// The amount that `free` may not drop below when withdrawing for *anything except transaction
    /// fee payment*.
    pub misc_frozen: u128,
    /// The amount that `free` may not drop below when withdrawing specifically for transaction
    /// fee payment.
    pub fee_frozen: u128,
}
