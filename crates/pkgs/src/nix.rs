use crate::PackageManager;
use anyhow::Error;
use std::{env, process::Command};
use users::get_current_username;

pub struct Nix {}

impl Nix {
    pub fn new() -> Self {
        Self {}
    }
}

impl PackageManager for Nix {
    fn install(&self, _name: &str) -> Result<(), Error> {
        self.setup()?;
        todo!();
    }

    fn uninstall(&self, _name: &str) -> Result<(), Error> {
        todo!()
    }

    fn setup(&self) -> Result<(), Error> {
        let user = match get_current_username() {
            Some(user) => user.to_string_lossy().to_string(),
            None => "root".to_string(),
        };

        env::set_var("USER", user);
        env::set_var(
            "PATH",
            format!(
                "{}:{}",
                env::var("PATH")?,
                "/nix/var/nix/profiles/default/bin"
            ),
        );
        let mut child = Command::new("sh")
            .arg("-c")
            .arg("type systemctl > /dev/null")
            .spawn()?;
        let status = child.wait()?;
        let init = match status.code() {
            Some(0) => "",
            _ => "--init none",
        };

        let linux = match std::env::consts::OS {
            "linux" => format!("linux --extra-conf 'sandbox = false' {}", init),
            _ => "".to_string(),
        };
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(format!("type nix > /dev/null || curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install {}", linux))
            .spawn()?;
        child.wait()?;

        let mut child = Command::new("sh")
            .arg("-c")
            .arg(format!("type nix > /dev/null || curl --proto '=https' --tlsv1.2 -sSf -L https://install.determinate.systems/nix | sh -s -- install {} --no-confirm", linux))
            .spawn()?;
        child.wait()?;
        Ok(())
    }
}
