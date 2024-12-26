use clap::{Parser, Subcommand};

mod main_ca;

#[derive(Parser)]
pub struct Arguments {
	#[command(subcommand)]
	command: Command,
}

#[derive(Subcommand)]
pub enum Command {
	/// Issue CA certificate.
	Ca(main_ca::Arguments),
}
fn main() -> Result<(), i32> {
	let args = Arguments::parse();

	match args.command {
		Command::Ca(args) => main_ca::main(args),
	}
}
