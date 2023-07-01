fn main() {
	let mut args = std::env::args();
	args.next().expect("no executable name");
	match args.next().as_deref() {
		None => {
			eprintln!("Usage: xtask <command>");
			eprintln!("Available commands:");
			eprintln!("  trunk *");
		}
		Some("trunk") => {
			let status = std::process::Command::new("trunk")
				.args(args)
				.status()
				.expect("failed to execute trunk");
			std::process::exit(status.code().unwrap_or(1));
		}
		Some(cmd) => eprintln!("Unknown command: {}", cmd),
	}
	println!("Hello, world!");
}
