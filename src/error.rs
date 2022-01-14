use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("The Operation cannot be performed due to insufficient funds")]
    InsufficientFunds {},
    #[error("The Validator does not exist in the Blockchain")]
    ValidatorDoesNotExist {},
    #[error("The Real Estate does not Exist")]
    RealEstateDoesNotExist {},
    

    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
}
