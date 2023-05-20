use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::Map;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct UserInfo {
    pub user: Addr,
    pub expires: Timestamp,
}
pub const USERS: Map<&str, UserInfo> = Map::new("users");
