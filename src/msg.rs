use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Boilerplate {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Returns BoilerplateResponse
    #[returns(BoilerplateResponse)]
    Boilerplate {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct BoilerplateResponse {}
