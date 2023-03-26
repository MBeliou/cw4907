use cosmwasm_std::{Addr, DepsMut};

use crate::state::USERS;

/*

    function _beforeTokenTransfer(
        address from,
        address to,
        uint256 tokenId
    ) internal virtual override{
        super._beforeTokenTransfer(from, to, tokenId);

        if (from != to && _users[tokenId].user != address(0)) {
            delete _users[tokenId];
            emit UpdateUser(tokenId, address(0), 0);
        }
    }
}
 */
fn before_token_transfer(deps: DepsMut, from: Addr, to: Addr, token_id: String) {
    if from != to {
        USERS.remove(deps.storage, token_id.as_str());
        // TODO: add event
    }
}
