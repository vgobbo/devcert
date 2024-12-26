use chrono::Duration;
use clap::Args;
use duration_flex::DurationFlex;
use exitcode::ExitCode;
use gethostname::gethostname;

use crate::ca;

#[derive(Args, Clone)]
pub struct Arguments {
	/// How long in the future should the certificate expire.
	#[arg(long, default_value_t = Arguments::default().ttl)]
	pub ttl: DurationFlex,

	/// ON to be used on the CA.
	#[arg(long, default_value_t = Arguments::default().on)]
	pub on: String,

	/// CN to be used on the CA.
	#[arg(long, default_value_t = Arguments::default().cn)]
	pub cn: String,

	/// Output file name (without extension).
	#[arg(long, default_value_t = Arguments::default().name)]
	pub name: String,
}

pub fn main(args: Arguments) -> Result<(), ExitCode> {
	match ca::generate(args.clone()) {
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
			name: "ca_cert".to_owned(),
		}
	}
}

impl From<Arguments> for ca::CaParameters {
	fn from(val: Arguments) -> Self {
		ca::CaParameters { ttl: val.ttl.into(), on: val.on, cn: val.cn }
	}
}
