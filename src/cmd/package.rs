use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind, Result};

pub fn generate() -> Result<String> {
    let path = get_current_path()?;

    let package_json = PackageJSON {
        name: path.as_str(),
        version: "0.0.1",
        description: "",
        main: "index.js",
        scripts: Scripts {
            test: "echo \"Error: no test specified\" && exit 1",
        },
        keywords: Vec::new(),
        author: "",
        license: "ISC",
    };

    let serialized = serde_json::to_string_pretty(&package_json)?;
    return Ok(serialized);
}

fn get_current_path() -> Result<String> {
    let dir = std::env::current_dir()?;

    if let Some(path) = dir.file_name() {
        if let Some(p) = path.to_str() {
            return Ok(p.to_owned());
        }
    }

    return Err(Error::new(ErrorKind::InvalidInput, "invalid path"));
}

#[derive(Serialize, Deserialize)]
struct PackageJSON<'a> {
    name: &'a str,
    version: &'a str,
    description: &'a str,
    main: &'a str,
    scripts: Scripts<'a>,
    keywords: Vec<&'a str>,
    author: &'a str,
    license: &'a str,
}

#[derive(Serialize, Deserialize)]
struct Scripts<'a> {
    test: &'a str,
}
