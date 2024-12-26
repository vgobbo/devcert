use std::path::PathBuf;

use chrono::Duration;
use clap::Args;
use duration_flex::DurationFlex;
use exitcode::ExitCode;
use gethostname::gethostname;

use crate::cert;
use crate::cert::CertParameters;
use crate::certificate_key_pair::CertificateKeyPair;

#[derive(Args, Clone)]
pub struct Arguments {
	/// How long in the future should the certificate expire.
	#[arg(long, default_value_t = Arguments::default().ttl)]
	pub ttl: DurationFlex,

	/// ON (Organization Name) to be used on the certificate.
	#[arg(long, default_value_t = Arguments::default().on)]
	pub on: String,

	/// CN (Common Name) to be used on the certificate.
	#[arg(long, default_value_t = Arguments::default().cn)]
	pub cn: String,

	/// CA to be used for the certificate (without extension).
	#[arg(long, default_value_t = Arguments::default().ca)]
	pub ca: String,

	/// Output file name (without extension).
	#[arg(long, default_value_t = Arguments::default().name)]
	pub name: String,

	/// Do not include localhost as SAN.
	#[arg(long, default_value_t = Arguments::default().no_localhost)]
	pub no_localhost: bool,

	/// Do not include hostname as SAN.
	#[arg(long, default_value_t = Arguments::default().no_hostname)]
	pub no_hostname: bool,

	/// Include these names as SAN (Subject Alternative Name).
	#[arg(long, num_args(0..))]
	pub sans: Vec<String>,
}

pub fn main(args: Arguments) -> Result<(), ExitCode> {
	let ca = match CertificateKeyPair::try_from(PathBuf::from(args.ca.as_str())) {
		Ok(cpk) => cpk,
		Err(e) => {
			eprintln!("{e}");
			return Err(e.into());
		},
	};

	let mut sans = args.sans.clone();
	if !args.no_hostname {
		sans.push(gethostname().as_os_str().to_string_lossy().to_string());
	}
	if !args.no_localhost {
		sans.push("localhost".to_string());
	}

	let cert_params = CertParameters { ttl: args.ttl.into(), on: args.on, cn: args.cn, ca, sans };

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
			no_hostname: false,
			no_localhost: false,
			sans: vec![],
		}
	}
}
