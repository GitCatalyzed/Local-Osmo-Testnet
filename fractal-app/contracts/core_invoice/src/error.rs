use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.

    #[error("Invoice has no value")]
    NoInvoiceValue{},

    #[error("Invoice has no date due")]
    NoDateDueValue{},

    #[error("Paying address doesn't match expected payer")]
    IncorrectPayer{},

    #[error("Payment value is negative")]
    InvalidPaymentValue{},

    #[error("Incorrect Date Format")]
    InvalidDate{},


}
