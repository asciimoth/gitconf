// Gitconf by DomesticMoth
//
// To the extent possible under law, the person who associated CC0 with
// gitconf has waived all copyright and related or neighboring rights
// to gitconf.
//
// You should have received a copy of the CC0 legalcode along with this
// work.  If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
use std::collections::HashMap;
use std::process::Command;
use which::which;
use serde::Deserialize;

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub strict: bool,
    pub strict_git: bool,
    pub select_profile_on_first_use: bool,
    pub show_current_profile: bool,
    pub interactive: bool,
    pub config: HashMap<String, String>,
}

impl Config {
    pub fn apply(&self) -> bool{
        let git = match which("git") {
            Ok(git) => git.into_os_string().into_string().unwrap(),
            Err(e) => {
                log::error!("Cannot find git command : {:?}", e);
                return false
            }
        };
        if self.strict_git {
            let out = match Command::new(git.clone()).arg("config").arg("--list").output() {
                Ok(out) => out,
                Err(e) => {
                    log::error!("Cannot get git configuretion {:?}", e);
                    return false
                }
            };
            let out =  String::from_utf8_lossy(&out.stdout);
            for line in out.lines(){
                let (config, _) = match line.split_once("=") {
                    Some((a, b)) => (a, b),
                    None => { continue },
                };
                if config.starts_with("core.") { continue }
                if config.starts_with("remote.") { continue }
                if config.starts_with("branch.") { continue }
                if let Err(e) = Command::new(git.clone())
                                            .arg("config")
                                            .arg("--unset-all")
                                            .arg(config)
                                            .output(){
                    log::error!("Cannot unset git config {:?}", e);
                    return false
                }
                if let Err(e) = Command::new(git.clone())
                                            .arg("config")
                                            .arg(config)
                                            .arg("")
                                            .output(){
                    log::error!("Cannot unset git config {:?}", e);
                    return false
                }
            }
        }
        for (config, value) in self.config.clone().into_iter() {
            if let Err(e) = Command::new(git.clone())
                                        .arg("config")
                                        .arg("--unset-all")
                                        .arg(config.clone())
                                        .output(){
                log::error!("Cannot unset git config {:?}", e);
                return false
            }
            if let Err(e) = Command::new(git.clone())
                                        .arg("config")
                                        .arg(config)
                                        .arg(value)
                                        .output(){
                log::error!("Cannot set git config {:?}", e);
                return false
            }
        }
        true
    }
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct OptionConfig {
    #[serde(default, rename = "Strict")]
    strict: Option<bool>,
    #[serde(default, rename = "StrictGit")]
    strict_git: Option<bool>,
    #[serde(default, rename = "SelectProfileOnFirstUse")]
    select_profile_on_first_use: Option<bool>,
    #[serde(default, rename = "ShowCurrentProfile")]
    show_current_profile: Option<bool>,
    #[serde(default, rename = "Interactive")]
    interactive: Option<bool>,
    #[serde(default, rename = "Config")]
    config: Option<HashMap<String, String>>,
}

impl OptionConfig {
    pub fn new() -> Self {
        Self{
            strict: None,
            strict_git: None,
            select_profile_on_first_use: None,
            show_current_profile: None,
            interactive: None,
            config: None,
        }
    }
    pub fn to_config(&self) -> Config {
        let mut default_config = HashMap::<String, String>::new();
        default_config.insert(String::from("user.name"), String::from("John Doe"));
        default_config.insert(String::from("user.email"), String::from(""));
        Config{
            strict: self.strict.unwrap_or(false),
            strict_git: self.strict_git.unwrap_or(true),
            select_profile_on_first_use: self.select_profile_on_first_use.unwrap_or(false),
            show_current_profile: self.show_current_profile.unwrap_or(true),
            interactive: self.interactive.unwrap_or(false),
            config: self.config.clone().unwrap_or(default_config),
        }
    }
    pub fn merge(&mut self, other: &OptionConfig) {
        if let Some(true) = other.strict {
            self.strict = other.strict;
            self.strict_git = other.strict_git;
            self.select_profile_on_first_use = other.select_profile_on_first_use;
            self.show_current_profile = other.show_current_profile;
            self.interactive = other.interactive;
            self.config = other.config.clone();
        }else{
            if let Some(v) = other.strict {
                self.strict = Some(v)
            }
            if let Some(v) = other.strict_git {
                self.strict_git = Some(v)
            }
            if let Some(v) = other.select_profile_on_first_use {
                self.select_profile_on_first_use = Some(v)
            }
            if let Some(v) = other.show_current_profile {
                self.show_current_profile = Some(v)
            }
            if let Some(v) = other.interactive {
                self.interactive = Some(v)
            }
            if let Some(other_config) = other.config.clone() {
                // Merge configs
                let mut config = match &self.config {
                    Some(self_config) => { self_config.clone() }
                    None => { HashMap::<String, String>::new() }
                };
                for (key, value) in other_config.into_iter() {
                    config.insert(key, value);
                }
                self.config = Some(config);
            }
        }
    }
}

#[cfg(test)]
mod tests_option_config {
    use crate::*;
    #[test]
    pub fn test_to_config() {
        let mut config = HashMap::new();
        config.insert(String::from("user.name"), String::from("John Doe"));
        let option_config = OptionConfig{
            strict: Some(false),
            strict_git: Some(true),
            select_profile_on_first_use: Some(false),
            show_current_profile: Some(true),
            interactive: None,
            config: Some(config.clone()),
        };
        let config = Config{
            strict: false,
            strict_git: true,
            select_profile_on_first_use: false,
            show_current_profile: true,
            interactive: false,
            config: config,
        };
        assert_eq!(option_config.to_config(), config);
    }
    #[test]
    pub fn test_parcing_none(){
        let correct = OptionConfig::new();
        let parced: OptionConfig = toml::from_str("\n").unwrap();
        assert_eq!(correct, parced);
    }
    #[test]
    pub fn test_parcing(){
        let mut config = HashMap::new();
        config.insert(String::from("user.name"), String::from("John Doe"));
        let correct = OptionConfig{
            strict: Some(false),
            strict_git: Some(true),
            select_profile_on_first_use: Some(false),
            show_current_profile: Some(true),
            interactive: Some(false),
            config: Some(config),
        };
        let parced: OptionConfig = toml::from_str(r#"
            Strict = false
            StrictGit = true
            SelectProfileOnFirstUse = false
            ShowCurrentProfile = true
            Interactive = false
            Config = { "user.name" = "John Doe" }
        "#).unwrap();
        assert_eq!(correct, parced);
    }
    #[test]
    pub fn test_merge(){
        let mut config = HashMap::new();
        config.insert(String::from("b"), String::from("2"));
        config.insert(String::from("c"), String::from("3"));
        let correct = OptionConfig{
            strict: Some(false),
            strict_git: None,
            select_profile_on_first_use: Some(false),
            show_current_profile: None,
            interactive: Some(true),
            config: Some(config),
        };
        let mut config = HashMap::new();
        config.insert(String::from("a"), String::from("1"));
        let first = OptionConfig{
            strict: Some(true),
            strict_git: Some(true),
            select_profile_on_first_use: Some(true),
            show_current_profile: Some(true),
            interactive: Some(true),
            config: Some(config),
        };
        let mut config = HashMap::new();
        config.insert(String::from("b"), String::from("2"));
        let second = OptionConfig{
            strict: Some(true),
            strict_git: None,
            select_profile_on_first_use: None,
            show_current_profile: None,
            interactive: None,
            config: Some(config),
        };
        let mut config = HashMap::new();
        config.insert(String::from("c"), String::from("3"));
        let third = OptionConfig{
            strict: Some(false),
            strict_git: None,
            select_profile_on_first_use: Some(false),
            show_current_profile: None,
            interactive: Some(true),
            config: Some(config),
        };
        let mut merged = OptionConfig::new();
        merged.merge(&first);
        merged.merge(&second);
        merged.merge(&third);
        assert_eq!(correct, merged);
    }
}
