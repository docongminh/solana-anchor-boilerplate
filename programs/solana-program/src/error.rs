use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Example define error")]
    ExampleError,
}