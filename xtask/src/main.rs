use semver::{Version, VersionReq};

const LEPTOSFMT_REQUIRED_VERSION: &str = "^0.1.12";

fn main() {
	let mut args = std::env::args();
	args.next().expect("no executable name");
	match args.next().as_deref() {
		None => {
			eprintln!("Usage: xtask <command>");
			eprintln!("Available commands:");
			eprintln!("  trunk * (same as running trunk directly");
			eprintln!(
				"  fmt (runs both rustfmt and leptosfmt, installs the latter if it doesn't exist via cargo install)"
			);
		}
		Some("trunk") => {
			let status = std::process::Command::new("trunk")
				.args(args)
				.status()
				.expect("failed to execute trunk");
			std::process::exit(status.code().unwrap_or(1));
		}
		Some("fmt") => {
			let leptos_fmt_version = std::process::Command::new("leptosfmt")
				.arg("--version")
				.output()
				.unwrap_or_else(|err| {
					eprintln!(
						"leptosfmt failed to execute, installing via `cargo install leptosfmt`, error was: {err}"
					);
					let status = std::process::Command::new("cargo")
						.arg("install")
						.arg("leptosfmt")
						.status()
						.expect("failed to execute `cargo install leptosfmt`");
					if !status.success() {
						eprintln!("`cargo install leptosfmt` failed with status {status}");
						std::process::exit(status.code().unwrap_or(1));
					}
					std::process::Command::new("leptosfmt")
						.arg("--version")
						.output()
						.expect("leptosfmt failed to execute, please remove leptosfmt as it is corrupt")
				});
			let leptos_version_output = String::from_utf8(leptos_fmt_version.stdout)
				.expect("leptosfmt output was not valid utf8, please remove leptosfmt as it is corrupt");
			let leptos_version_error = String::from_utf8(leptos_fmt_version.stderr)
				.expect("leptosfmt error was not valid utf8, please remove leptosfmt as it is corrupt");
			if !leptos_version_error.is_empty() {
				eprintln!("leptosfmt status: {}", leptos_fmt_version.status);
				eprintln!("leptosfmt standard output:");
				eprintln!("{leptos_version_output}");
				eprintln!("leptosfmt error output:");
				eprintln!("{leptos_version_error}");
				std::process::exit(1);
			}
			match leptos_version_output.split_once(' ') {
				Some(("leptosfmt", version)) => {
					let leptosfmt_version = Version::parse(version.trim()).expect("leptosfmt version was invalid");
					if !VersionReq::parse(LEPTOSFMT_REQUIRED_VERSION)
						.expect("leptosfmt required version is invalid")
						.matches(&leptosfmt_version)
					{
						eprintln!("leptosfmt version {leptosfmt_version} does not match required version {LEPTOSFMT_REQUIRED_VERSION}");
						eprintln!("Please run `cargo install leptosfmt` to install a correct version or run `cargo uninstall leptosfmt` and then run `cargo xtask fmt` again to install the latest version");
						std::process::exit(1);
					}
				}
				_invalid => {
					eprintln!("leptosfmt --version out was invalid: {leptos_version_output}");
					std::process::exit(1);
				}
			}
			// `leptosfmt` is installed and of an acceptable version at this point
			// Let's make sure that `cargo check` isn't erroring first
			if !std::process::Command::new("cargo")
				.arg("check")
				.status()
				.expect("failed to execute `cargo check`")
				.success()
			{
				eprintln!("`cargo check` reported an error, aborting");
				std::process::exit(1);
			}
			// Run leptosfmt first because it doesn't follow rustfmt.toml correctly, like it forces spaces and such...
			// Then run `cargo fmt` after to format everything else and fix up leptosfmt's mistakes, but leaving
			// what leptosfmt did within the macro
			std::process::Command::new("leptosfmt")
				.args(["--config-file", "rustfmt.toml", "--", "src"])
				.status()
				.expect("failed to execute `leptosfmt`");
			std::process::Command::new("cargo")
				.arg("fmt")
				.status()
				.expect("failed to execute `cargo fmt``");
		}
		Some(cmd) => eprintln!("Unknown command: {}", cmd),
	}
}
