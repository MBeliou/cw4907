use cosmwasm_std::{Addr, Event, Storage, Timestamp};

use crate::{
    contract::{ToEvent, UpdateUserEvent},
    state::USERS,
};

pub fn before_token_transfer(
    storage: &mut dyn Storage,
    from: Addr,
    to: Addr,
    token_id: String,
) -> Option<Event> {
    if from != to && USERS.has(storage, token_id.as_str()) {
        USERS.remove(storage, token_id.as_str());
        let event = UpdateUserEvent {
            expires: Timestamp::from_nanos(0),
            user: Addr::unchecked("0"),
            token_id,
        };
        return Some(event.to_event());
    }
    None
}
