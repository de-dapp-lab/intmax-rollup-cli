use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Given aggregator URL is invalid.")]
    InvalidAggregatorURL,

    #[error("Given aggregator URL is valid but is an incompatible version. If you get this error, synchronizing this CLI to the latest version may solve the problem. For more information, see https://github.com/InternetMaximalism/intmax-rollup-cli#update .")]
    IncompatibleVersionAggregatorURL,

    #[error("token address must be your user address: {0}")]
    MustYourAddress(String),

    #[error("deposit amount must be a positive integer less than 2^56: {0}")]
    NotPositiveIntegerDeposit(u64),

    #[error("sending amount must be a positive integer less than 2^56: {0}")]
    NotPositiveIntegerSending(u64),

    #[error("unexpected response in health check from {api_path:?}: {error_message:?}")]
    FailedHealthCheck {
        api_path: String,
        error_message: String,
    },

    #[error("unexpected response in get latest block from {api_path:?}: {error_message:?}")]
    FailedGetLatestBlock {
        api_path: String,
        error_message: String,
    },

    #[error("unexpected response in get blocks from {api_path:?}: {error_message:?}")]
    FailedGetBlocks {
        api_path: String,
        error_message: String,
    },

    #[error("unexpected response in get block details from {api_path:?}: {error_message:?}")]
    FailedGetBlockDetails {
        api_path: String,
        error_message: String,
    },

    #[error("unexpected response in get transaction inclusion witness from {api_path:?}: {error_message:?}")]
    FailedGetTransactionInclusionWitness {
        api_path: String,
        error_message: String,
    },

    #[error(
        "unexpected response in get merge transaction witness from {api_path:?}: {error_message:?}"
    )]
    FailedGetMergeTransactionWitness {
        api_path: String,
        error_message: String,
    },

    #[error("unexpected response in get possession proof from {api_path:?}: {error_message:?}")]
    FailedGetPossessionProof {
        api_path: String,
        error_message: String,
    },

    #[error("nothing to do")]
    NothingToDo,

    #[error("output asset amount is too much. input {input:?} < output {output:?}")]
    InsufficientOutputAmount { input: u64, output: u64 },

    #[error("too many fragments of assets. input_length {input_length:?} > max {max:?}")]
    TooManyInputAssets { input_length: usize, max: usize },

    #[error(
        "too many destinations and token kinds. output_length {output_length:?} > max {max:?}"
    )]
    TooManyOutputAssets { output_length: usize, max: usize },
}
