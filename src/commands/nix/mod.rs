use self::{
    install::{nix_install, NixInstallCommand},
    list::{nix_list, NixListCommand},
    remove::{nix_remove, NixRemoveCommand},
    upgrade::{nix_upgrade, NixUpgradeCommand},
};
use anyhow::Result;
use clap::Parser;

mod install;
mod link_generator;
mod list;
mod remove;
mod shell;
mod upgrade;

macro_rules! nix_info {
    ($output:expr) => {
        if !$output.stdout.is_empty() {
            info!("{}", String::from_utf8_lossy(&$output.stdout));
        }
        if !$output.stderr.is_empty() {
            info!("{}", String::from_utf8_lossy(&$output.stderr));
        }
    };
}
pub(crate) use nix_info;

pub(crate) const NIX_CMD: &str = "nix";
// pub(crate) const SHELL_ARG: &str = "shell";
pub(crate) const PROFILE_ARG: &str = "profile";
pub(crate) const UNLOCKED_FLAKE_REF: &str = ".*";
pub(crate) const PROFILE_INSTALL_ARGS: &[&str; 2] = &[PROFILE_ARG, "install"];
pub(crate) const PROFILE_LIST_ARGS: &[&str; 2] = &[PROFILE_ARG, "list"];
pub(crate) const PROFILE_REMOVE_ARGS: &[&str; 2] = &[PROFILE_ARG, "remove"];
pub(crate) const PROFILE_UPGRADE_ARGS: &[&str; 2] = &[PROFILE_ARG, "upgrade"];
pub(crate) const PRIORITY_FLAG: &str = "--priority";
pub(crate) const FUEL_NIX_LINK: &str = "github:fuellabs/fuel.nix";

#[derive(Debug, Parser)]
pub enum NixCommand {
    /// Install a distributable toolchain or component.
    Install(NixInstallCommand),
    /// Uninstall a toolchain or component by providing its index,
    /// unlocked attribute path or nix store path.
    Remove(NixRemoveCommand),
    /// Upgrade installed packages by index or unlocked attribute path
    /// with the latest version of the fuel.nix flake. Upgrades all
    /// installed packages if no index or path is provided.
    Upgrade(NixUpgradeCommand),
    // /// Opens a new bash shell instace with specified toolchain or component available on `$PATH`.
    // Shell(NixShellCommand),
    /// Lists the installed packages by index, unlocked attribute path,
    /// locked attribute path and nix store path, respectively.
    List(NixListCommand),
}

pub fn exec(command: NixCommand) -> Result<()> {
    match command {
        NixCommand::Install(command) => nix_install(command),
        NixCommand::Remove(command) => nix_remove(command),
        NixCommand::Upgrade(command) => nix_upgrade(command),
        // NixCommand::Shell(command) => nix_shell(command),
        NixCommand::List(_command) => nix_list(_command),
    }
}
