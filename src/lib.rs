use clap::{Args, Parser, Subcommand};
pub mod cmd;

#[derive(Parser)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a package
    Init,
    /// Install all dependencies
    Install(InstallFlags),
    /// Add a new dependency
    Add(AddFlags),
}

#[derive(Args)]
pub struct InstallFlags {
    /// Do not install packages listed under "devDependencies"
    #[arg(short = 'P', long)]
    prod: bool,
}

#[derive(Args)]
pub struct AddFlags {
    /// List of packages to install
    #[arg(required = true)]
    packages: Vec<String>,

    #[arg(short = 'D', long)]
    /// Save to "devDependencies"
    save_dev: bool,

    #[arg(short = 'P', long, default_value_t = true)]
    /// Save to "dependencies". Default behaviour
    save_prod: bool,
}
