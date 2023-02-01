use super::{AddFlags, InstallFlags};

use std::fs;
use std::io::{Error, ErrorKind, Result};

mod package;

pub fn add(add: &AddFlags) {
    for package in &add.packages {
        print!("{package} ")
    }
    println!("{} {}", add.save_dev, add.save_prod);
}

pub fn install(install: &InstallFlags) {
    println!("{}", install.prod);
}

pub fn init() -> Result<()> {
    let file_exists = fs::metadata("package.json").is_ok();

    if file_exists {
        return Err(Error::new(
            ErrorKind::AlreadyExists,
            "package.json already exists.",
        ));
    }

    let serialized = package::generate()?;
    fs::write("package.json", serialized)?;

    return Ok(());
}
