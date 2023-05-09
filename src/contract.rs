use cosmwasm_schema::cw_serde;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Empty, Env, Event, MessageInfo, Response, StdResult,
    Timestamp,
};
use cw2::set_contract_version;
use cw721_base::state::TokenInfo;
use cw721_base::{ContractError, Extension};
//use cw_multi_test::Contract;

use crate::messages::{CW4709ExecuteMsg, CW4709QueryMsg, GetCountResponse, InstantiateMsg};

use self::query::{user_expiration, user_of};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw4907";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type Cw4907Contract<'a> =
    cw721_base::Cw721Contract<'a, Extension, Empty, CW4709ExecuteMsg, CW4709QueryMsg>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension, CW4709ExecuteMsg>;
pub type QueryMsg = cw721_base::QueryMsg<CW4709QueryMsg>;

pub trait Approval {
    /// Returns if the message sender is the owner or has an approval for a given token. This includes the operators

    fn _is_approved_or_owner(
        &self,
        deps: Deps,
        env: &Env,
        info: &MessageInfo,
        token: &TokenInfo<Extension>,
    ) -> bool;
}

impl Approval for Cw4907Contract<'_> {
    fn _is_approved_or_owner(
        &self,
        deps: Deps,
        env: &Env,
        info: &MessageInfo,
        token: &TokenInfo<Extension>,
    ) -> bool {
        if token.owner == info.sender {
            return true;
        }

        if token
            .approvals
            .iter()
            .any(|approval| approval.spender == info.sender && !approval.is_expired(&env.block))
        {
            return true;
        }

        let operator = self
            .operators
            .may_load(deps.storage, (&token.owner, &info.sender))
            .unwrap();
        match operator {
            Some(expiration) => {
                if expiration.is_expired(&env.block) {
                    false
                } else {
                    true
                }
            }
            None => false,
        }
    }
}

#[cw_serde]
pub struct UpdateUserEvent {
    pub token_id: String,
    pub user: Addr,
    pub expires: Timestamp,
}

pub trait ToEvent {
    fn to_event(self) -> Event;
}

impl ToEvent for UpdateUserEvent {
    fn to_event(self) -> Event {
        Event::new("update_user").add_attributes(vec![
            ("tokenId", self.token_id),
            ("user", self.user.to_string()),
            ("expires", self.expires.to_string()),
        ])
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    /*
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
         */
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Cw4907Contract::default()
        .instantiate(deps, _env, info, msg)
        .unwrap();

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Extension { msg } => match msg {
            CW4709ExecuteMsg::SetUser {
                token_id,
                user,
                expires,
            } => execute::set_user(deps, info, _env, token_id, user, expires),
        },
        _ => Cw4907Contract::default()
            .execute(deps, _env, info, msg)
            .map_err(Into::into),
    }
}
pub mod execute {
    use cosmwasm_std::{Addr, Event, Timestamp};
    use cw721::Cw721Query;

    use crate::state::{UserInfo, USERS};

    use super::*;

    pub fn set_user(
        deps: DepsMut,
        info: MessageInfo,
        _env: Env,
        token_id: String,
        user: String,
        expires: Timestamp,
    ) -> Result<Response, ContractError> {
        let token = Cw4907Contract::default()
            .tokens
            .load(deps.storage, &token_id)?;

        if !Cw4907Contract::default()._is_approved_or_owner(deps.as_ref(), &_env, &info, &token) {
            return Err(ContractError::Unauthorized {});
        }

        let addr = deps.api.addr_validate(&user).unwrap();
        match USERS.save(
            deps.storage,
            token_id.as_str(),
            &UserInfo {
                expires: expires,
                user: addr,
            },
        ) {
            Ok(_) => (),
            Err(e) => return Err(ContractError::Std(e)),
        }

        let updateUserEvent = UpdateUserEvent {
            user: addr,
            expires: expires,
            token_id: token_id,
        };
        let event = updateUserEvent.to_event();
        Ok(Response::new().add_event(event))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Extension { msg } => match msg {
            CW4709QueryMsg::UserExpires { token_id } => {
                to_binary(&user_expiration(deps, token_id).unwrap())
            }
            CW4709QueryMsg::UserOf { token_id } => to_binary(&user_of(deps, token_id).unwrap()),
        },
        _ => Cw4907Contract::default()
            .query(deps, _env, msg)
            .map_err(Into::into),
    }
}

pub mod query {
    use cosmwasm_std::Timestamp;

    use crate::{
        messages::GetUserResponse,
        state::{UserInfo, USERS},
    };

    use super::*;

    pub fn user_expiration(deps: Deps, token_id: String) -> Result<Timestamp, ContractError> {
        let user = USERS.may_load(deps.storage, token_id.as_str()).unwrap();

        match user {
            Some(u) => Ok(u.expires),
            None => Err(ContractError::NoUser {}),
        }
    }

    pub fn user_of(deps: Deps, token_id: String) -> StdResult<GetUserResponse> {
        let user_info = USERS.may_load(deps.storage, token_id.as_str()).unwrap();
        Ok(GetUserResponse {
            user_info: user_info,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::messages::GetUserResponse;

    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, Addr, Timestamp};
    use cw721_base::MintMsg;

    const MINTER: &str = "minter";
    const CONTRACT_NAME: &str = "CW4709";
    const SYMBOL: &str = "CW4";

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            minter: MINTER.into(),
            name: CONTRACT_NAME.into(),
            symbol: SYMBOL.into(),
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        /*
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
         */
    }
    /*
       #[test]
       fn increment() {
           let mut deps = mock_dependencies();

           let msg = InstantiateMsg { count: 17 };
           let info = mock_info("creator", &coins(2, "token"));
           let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

           // beneficiary can release it
           let info = mock_info("anyone", &coins(2, "token"));
           let msg = ExecuteMsg::Increment {};
           let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

           // should increase counter by 1
           let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
           let value: GetCountResponse = from_binary(&res).unwrap();
           assert_eq!(18, value.count);
       }
    */

    const USER: &str = "user";
    #[test]
    fn set_user() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            minter: MINTER.into(),
            name: CONTRACT_NAME.into(),
            symbol: SYMBOL.into(),
        };
        let info = mock_info(MINTER, &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let mint_msg = ExecuteMsg::Mint(MintMsg {
            token_id: "0".into(),
            owner: Addr::unchecked(USER).into(),
            token_uri: None,
            extension: None,
        });

        let _res_mint = execute(deps.as_mut(), mock_env(), info.clone(), mint_msg).unwrap();

        // only the owner can set a user
        let msg = ExecuteMsg::Extension {
            msg: CW4709ExecuteMsg::SetUser {
                expires: Timestamp::from_seconds(1962301978),
                token_id: "0".to_string(),
                user: Addr::unchecked(USER).to_string(),
            },
        };

        let set_user_info = mock_info(Addr::unchecked(USER).as_str(), &coins(2, "token"));
        let _res = execute(deps.as_mut(), mock_env(), set_user_info, msg).unwrap();

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::Extension {
                msg: CW4709QueryMsg::UserOf {
                    token_id: "0".to_string(),
                },
            },
        )
        .unwrap();
        let value: GetUserResponse = from_binary(&res).unwrap();

        assert_eq!("user".to_string(), value.user_info.unwrap().user);
    }
}
