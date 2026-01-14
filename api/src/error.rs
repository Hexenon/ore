use steel::*;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum OreError {
    #[error("Amount too small")]
    AmountTooSmall = 0,

    #[error("Not authorized")]
    NotAuthorized = 1,

    #[error("Missing payer signature")]
    MissingPayerSignature = 2,

    #[error("Mint address mismatch")]
    MintAddressMismatch = 3,

    #[error("LP pool PDA mismatch")]
    LpPoolPdaMismatch = 4,

    #[error("LP pool already initialized")]
    LpPoolAlreadyInitialized = 5,
}

error!(OreError);
