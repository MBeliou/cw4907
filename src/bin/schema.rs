use cosmwasm_schema::write_api;

use cw4907::messages::{CW4709ExecuteMsg, InstantiateMsg, CW4709QueryMsg};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: CW4709ExecuteMsg,
        query: CW4709QueryMsg,
    }
}
