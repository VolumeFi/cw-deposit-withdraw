use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub total_supply: Uint128,
}

pub const BALANCES: Map<&[u8], Uint128> = Map::new("balance");

pub const STATE: Item<State> = Item::new("state");

pub const ANCHOR_LIQUIDATION_QUEUE_ADDR: &str = "terra1e25zllgag7j9xsun3me4stnye2pcg66234je3u";

pub const B_LUNA_ADDR: &str = "terra1kc87mu460fwkqte29rquh4hc20m54fxwtsx7gp";

pub const PRICE_ORACLE_ADDR: &str = "terra1cgg6yef7qcdm070qftghfulaxmllgmvk77nc7t";
