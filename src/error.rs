use solana_program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum ClmmError {
    #[error("Invalid tick range: lower must be less than upper")]
    InvalidTickRange,
    #[error("Tick out of bounds")]
    TickOutOfBounds,
    #[error("Invalid tick spacing")]
    InvalidTickSpacing,
    #[error("Tick not aligned to tick spacing")]
    TickNotAligned,
    #[error("Invalid sqrt price")]
    InvalidSqrtPrice,
    #[error("Insufficient liquidity")]
    InsufficientLiquidity,
    #[error("Slippage exceeded")]
    SlippageExceeded,
    #[error("Pool not initialized")]
    PoolNotInitialized,
    #[error("Pool already initialized")]
    PoolAlreadyInitialized,
    #[error("Position not found")]
    PositionNotFound,
    #[error("Invalid token order: token0 must be less than token1")]
    InvalidTokenOrder,
    #[error("Math overflow")]
    MathOverflow,
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Invalid account owner")]
    InvalidAccountOwner,
    #[error("Invalid account data")]
    InvalidAccountData,
    #[error("Insufficient funds")]
    InsufficientFunds,
}

impl From<ClmmError> for ProgramError {
    fn from(e: ClmmError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
