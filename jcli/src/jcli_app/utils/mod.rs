mod account_id;
mod debug_flag;

pub mod host_addr;
pub mod io;
pub mod key_parser;
pub mod open_api_verifier;
pub mod output_format;
pub mod rest_api;

pub use self::account_id::AccountId;
pub use self::debug_flag::DebugFlag;
pub use self::host_addr::HostAddr;
pub use self::open_api_verifier::OpenApiVerifier;
pub use self::output_format::OutputFormat;
pub use self::rest_api::{RestApiResponse, RestApiResponseBody, RestApiSender};
use bech32;
use structopt::StructOpt;
use thiserror::Error;

#[derive(StructOpt)]
#[structopt(name = "utils", rename_all = "kebab-case")]
pub enum Utils {
    /// convert a bech32 with hrp n into a bech32 with prefix m
    Bech32Convert(Bech32ConvertArgs),
}

#[derive(StructOpt)]
pub struct Bech32ConvertArgs {
    /// the bech32 you want to convert
    #[structopt(name = "FROM_BECH32")]
    from_bech32: String,

    /// the new bech32 hrp you want to use
    #[structopt(name = "NEW_PREFIX")]
    new_hrp: String,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to convert bech32")]
    Bech32ConversionFailure(#[from] bech32::Error),
}

impl Utils {
    pub fn exec(self) -> Result<(), Error> {
        match self {
            Utils::Bech32Convert(convert_args) => {
                convert_prefix(convert_args.from_bech32, convert_args.new_hrp).map_err(|e| e.into())
            }
        }
    }
}

fn convert_prefix(from_addr: String, prefix: String) -> Result<(), Error> {
    let (_, d) = bech32::decode(&from_addr)?;
    let n = bech32::encode(&prefix, d)?;
    println!("{}", n);
    Ok(())
}
