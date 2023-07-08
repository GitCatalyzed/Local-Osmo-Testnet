use cosmwasm_schema::{cw_serde, QueryResponses};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Message type for `instantiate` entry_point
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {

        pub admin: Option<String>,

}

/// Message type for `execute` entry_point
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {

    CreateInvoice{
        //should payer_addr and payer_alias be linked in relational DB?
        payer_addr: String,
        payer_alias: String,
        invoice_id: String,
        invoiced_value: String,
        date_due: String,
        pay_unit: String,
        receipt_unit: String,
    },

    PayInvoice{
        invoice_id: String,
        payer_alias: String,
        payment_amount: String,
        pay_unit: String,
    }

}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

/// Message type for `query` entry_point
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, QueryResponses)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // This example query variant indicates that any client can query the contract
    // using `YourQuery` and it will return `YourQueryResponse`
    // This `returns` information will be included in contract's schema
    // which is used for client code generation.
    //
    // #[returns(YourQueryResponse)]
    // YourQuery {},
    #[returns(AllInvoicessResponse)]
    AllInvoices{},
    
    #[returns(InvoiceResponse)]
    Invoice{
        invoice_id: String,
    },

    #[returns(PaymentResponse)]
    Payment{
        invoice_id: String,
        address: String,
    },

}

// We define a custom struct for each query response
// #[cw_serde]
// pub struct YourQueryResponse {}
// Needed import
use crate::state::{Invoice, Payment};
// Previous code omitted
// Needed macro derivations
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
//#[cw_serde]
pub struct AllInvoicessResponse {
    pub invoices: Vec<Invoice>,
}

// Previous code omitted
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct InvoiceResponse {
    pub invoice: Option<Invoice>,
}

// Previous code omitted
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct PaymentResponse {
    pub payment: Option<Payment>,
}