use std::error::Error;
use std::process::{exit, Command};

fn main() -> Result<(), Box<dyn Error>> {
    let status = Command::new("buf").arg("generate").status().unwrap();

    if !status.success() {
        exit(status.code().unwrap_or(-1))
    }

    Ok(())
}
