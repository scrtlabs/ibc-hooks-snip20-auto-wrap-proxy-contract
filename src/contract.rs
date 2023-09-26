use cosmwasm_std::{
    entry_point, to_binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::msg::{ExecMsg, InitMsg, Snip20HandleMsg};

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InitMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[entry_point]
pub fn execute(_deps: DepsMut, _env: Env, info: MessageInfo, msg: ExecMsg) -> StdResult<Response> {
    match msg {
        ExecMsg::WrapDeposit {
            snip20_address,
            snip20_code_hash,
            recipient_address,
        } => Ok(Response::default().add_messages(vec![
            CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
                contract_addr: snip20_address.clone(),
                code_hash: snip20_code_hash.clone(),
                msg: to_binary(&Snip20HandleMsg::Deposit { padding: None }).unwrap(),
                funds: info.funds.clone(),
            }),
            CosmosMsg::Wasm(cosmwasm_std::WasmMsg::Execute {
                contract_addr: snip20_address,
                code_hash: snip20_code_hash,
                msg: to_binary(&Snip20HandleMsg::Transfer {
                    recipient: recipient_address,
                    amount: info.funds[0].amount,
                    memo: None,
                    padding: None,
                })
                .unwrap(),
                funds: vec![],
            }),
        ])),
    }
}
