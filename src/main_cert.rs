use std::fs;
use std::path::PathBuf;
use chrono::Duration;
use clap::Args;
use duration_flex::DurationFlex;
use exitcode::ExitCode;
use gethostname::gethostname;
use rcgen::{CertificateParams, KeyPair};
use crate::cert;
use crate::cert::CertParameters;
use crate::certificate_key_pair::{CertificateKeyPair, ErrorKind};

#[derive(Args, Clone)]
pub struct Arguments {
    /// How long in the future should the certificate expire.
    #[arg(long, default_value_t = Arguments::default().ttl)]
    pub ttl: DurationFlex,

    /// ON to be used on the certificate.
    #[arg(long, default_value_t = Arguments::default().on)]
    pub on: String,

    /// CN to be used on the certificate.
    #[arg(long, default_value_t = Arguments::default().cn)]
    pub cn: String,

    /// CA to be used for the certificate (without extension).
    #[arg(long, default_value_t = Arguments::default().ca)]
    pub ca: String,

    /// Output file name (without extension).
    #[arg(long, default_value_t = Arguments::default().name)]
    pub name: String,
}

pub fn main(args: Arguments) -> Result<(), ExitCode> {
    let ca = match CertificateKeyPair::try_from(PathBuf::from(args.ca.as_str())) {
        Ok(cpk) => cpk,
        Err(e) => {
            eprintln!("{e}");
            return Err(e.into());
        }
    };

    let cert_params = CertParameters {
        ttl: args.ttl.into(),
        on: args.on,
        cn: args.cn,
        ca,
        sans: vec![],
    };

    match cert::generate(cert_params) {
        Ok(ckp) => ckp.write(args.name.as_str()).map_err(|_| exitcode::IOERR),
        Err(rcgen::Error::PemError(_)) => Err(exitcode::IOERR),
        Err(_) => Err(exitcode::SOFTWARE),
    }
}

impl Default for Arguments {
    fn default() -> Self {
        Self {
            ttl: Duration::days(365).into(),
            on: gethostname().to_string_lossy().into_owned(),
            cn: gethostname().to_string_lossy().into_owned(),
            ca: "ca_cert".to_owned(),
            name: "cert".to_owned(),
        }
    }
}
