use std::process::ExitCode;

use clap::{Parser, Subcommand};

mod ca;
mod cert;
mod certificate_key_pair;
mod kp;
mod main_ca;
mod main_cert;

/// Easily generate certificates (and CA) to be used in development environments.
#[derive(Parser)]
pub struct Arguments {
	#[command(subcommand)]
	command: Command,
}

#[derive(Subcommand)]
pub enum Command {
	/// Issue CA certificate.
	Ca(main_ca::Arguments),

	/// Issue a certificate, signed by a CA.
	Cert(main_cert::Arguments),
}
fn main() -> ExitCode {
	let args = Arguments::parse();

	let result = match args.command {
		Command::Ca(args) => main_ca::main(args),
		Command::Cert(args) => main_cert::main(args),
	};

	let code = match result {
		Ok(_) => exitcode::OK,
		Err(e) => e,
	};
	ExitCode::from(code as u8)
}
