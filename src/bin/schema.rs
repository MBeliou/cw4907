use cosmwasm_schema::write_api;

use cw4907::messages::{CW4709ExecuteMsg, CW4709QueryMsg, InstantiateMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: CW4709ExecuteMsg,
        query: CW4709QueryMsg,
    }
}
