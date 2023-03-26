use crate::state::UserInfo;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, CustomMsg, Timestamp};
use cw721::Cw721ExecuteMsg;

/*
#[cw_serde]
pub struct InstantiateMsg {}
*/
pub type InstantiateMsg = cw721_base::InstantiateMsg;

#[cw_serde]
pub enum CW4709ExecuteMsg {
    SetUser {
        token_id: String,
        user: String,
        expires: Timestamp,
    },
}

impl CustomMsg for CW4709ExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum CW4709QueryMsg {
    // GetCount returns the current count as a json-encoded number
    //#[returns(GetCountResponse)]
    //GetCount {},
    #[returns(GetUserResponse)]
    UserOf { token_id: String },
    #[returns(GetUserExpiresResponse)]
    UserExpires { token_id: String },
}

/*
impl Default for CW4709QueryMsg {
    fn default() -> Self {
        CW4709QueryMsg::UserOf {}
    }
}
 */

impl CustomMsg for CW4709QueryMsg {}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}

#[cw_serde]
pub struct GetUserResponse {
    pub user_info: Option<UserInfo>,
}

#[cw_serde]
pub struct GetUserExpiresResponse {
    pub expires: Option<Timestamp>,
}
