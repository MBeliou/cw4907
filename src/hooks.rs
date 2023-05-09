use cosmwasm_std::{Addr, DepsMut, Event, Timestamp};

use crate::{contract::{UpdateUserEvent, ToEvent}, state::USERS};

pub fn before_token_transfer(deps: DepsMut, from: Addr, to: Addr, token_id: String) -> Option<Event> {
    if from != to && USERS.has(deps.storage, token_id.as_str()) {
        USERS.remove(deps.storage, token_id.as_str());
        // TODO: add event
        let event = UpdateUserEvent {
            expires: Timestamp::from_nanos(0),
            user: Addr::unchecked("0"),
            token_id: token_id,
        };
        return Some(event.to_event());
    }
    None
}
