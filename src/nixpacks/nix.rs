use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Pkg {
    pub name: String,
    pub overrides: HashMap<String, String>,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize, PartialEq, Clone, Default, Debug)]
pub struct NixConfig {
    pub pkgs: Vec<Pkg>,
    pub overlays: Vec<String>,
    pub nixpkgs_archive: Option<String>,
}

impl Pkg {
    pub fn new(name: &str) -> Pkg {
        Pkg {
            name: name.to_string(),
            overrides: HashMap::default(),
        }
    }

    pub fn to_nix_string(&self) -> String {
        if self.overrides.is_empty() {
            self.name.clone()
        } else {
            let override_string = self
                .overrides
                .iter()
                .map(|(name, value)| format!("{} = {};", name, value))
                .collect::<Vec<_>>()
                .join(" ");
            format!("({}.override {{ {} }})", self.name, override_string)
        }
    }

    pub fn set_override(mut self, name: &str, pkg: &str) -> Self {
        self.overrides.insert(name.to_string(), pkg.to_string());
        self
    }
}

impl NixConfig {
    pub fn new(pkgs: Vec<Pkg>) -> Self {
        NixConfig {
            pkgs,
            overlays: Vec::new(),
            nixpkgs_archive: None,
        }
    }

    pub fn add_pkgs(&mut self, new_pkgs: &mut Vec<Pkg>) {
        self.pkgs.append(new_pkgs);
    }

    pub fn add_overlay(&mut self, overlay: String) {
        self.overlays.push(overlay);
    }

    pub fn set_archive(&mut self, archive: String) {
        self.nixpkgs_archive = Some(archive);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_pkg_to_string() {
        assert_eq!(Pkg::new("cowsay").to_nix_string(), "cowsay".to_string());
    }

    #[test]
    fn test_pkg_single_override_to_string() {
        assert_eq!(
            Pkg::new("cowsay")
                .set_override("hello", "hello_1.1")
                .to_nix_string(),
            "(cowsay.override { hello = hello_1.1; })".to_string()
        );
    }
}