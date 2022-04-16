use std::path::PathBuf;
use std::collections::HashMap;
use serde::Deserialize;
use std::fs;
use log;
use simplelog;
use std::process::Command;
use which::which;
use std::env;
use std::{io, io::Write};
use std::os::unix::process::CommandExt;

pub struct PathIter{
    buf: PathBuf,
    end: bool,
    last: PathBuf,
}

impl PathIter {
    pub fn new(path: PathBuf) -> Self{
        Self{
            buf: path,
            end: false,
            last: PathBuf::from("/"),
        }
    }
    pub fn current() -> std::io::Result<Self> {
        let buf = std::env::current_dir()?;
        Ok(Self::new(buf))
    }
}

impl Iterator for PathIter {
    type Item = PathBuf;
    fn next(&mut self) -> Option<Self::Item> {
        loop{
            if self.end { return None }
            let mut ret = self.buf.clone();
            if let Some("/") = ret.to_str() {
                ret = PathBuf::from("/etc")
            }
            self.end = !self.buf.pop();
            if ret == self.last {
                continue
            } else {
                self.last = ret.clone();
            }
            return Some(ret)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    strict: bool,
    strict_git: bool,
    select_profile_on_first_use: bool,
    show_current_profile: bool,
    interactive: bool,
    config: HashMap<String, String>,
}

impl Config {
    fn apply(&self) -> bool{
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
    fn new() -> Self {
        Self{
            strict: None,
            strict_git: None,
            select_profile_on_first_use: None,
            show_current_profile: None,
            interactive: None,
            config: None,
        }
    }
    fn to_config(&self) -> Config {
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
    fn merge(&mut self, other: &OptionConfig) {
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

mod tests_option_config {
    #[test]
    fn test_to_config() {
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
    fn test_parcing_none(){
        let correct = OptionConfig::new();
        let parced: OptionConfig = toml::from_str("").unwrap();
        assert_eq!(correct, parced);
    }
    #[test]
    fn test_parcing(){
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
    fn test_merge(){
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

fn prepend<T>(v: Vec<T>, s: T) -> Vec<T>
where
    T: Clone,
{
    let s = vec![s];
    let mut tmp: Vec<_> = s.to_owned();
    tmp.extend(v);
    tmp
}

fn get_current_config_for_path(mut cur_path: PathBuf) -> (Config, Option<String>){
    let mut config_path: Option<String> = None;
    let mut opt = OptionConfig::new();
    let mut pathes = PathIter::new(cur_path.clone()).collect::<Vec<PathBuf>>();
    cur_path.push(".git");
    pathes = prepend(pathes, cur_path);
    for path in pathes.iter_mut().rev() {
        path.push(".gitconf");
        path.push("current");
    }
    for path in pathes.iter_mut().rev() {
        if let Ok(read_dir) = fs::read_dir(path.clone()) {
            let entrys: Vec<std::io::Result<std::fs::DirEntry>> = read_dir.collect();
            let mut files: Vec<PathBuf> = Vec::new();
            for entry in entrys {
                if let Ok(entry) = entry {
                    files.push(entry.path())
                }
            }
            if files.len() == 1 {
                let cur_conf: OptionConfig = match toml::from_str(
                    match fs::read_to_string(files[0].clone().into_os_string()){
                        Ok(s) => s,
                        Err(_) => {
                            log::warn!("Cannot read config {:?}", files[0]);
                            continue
                        }
                    }.as_str()
                ){
                    Ok(cur_conf) => cur_conf,
                    Err(_) => {
                        log::warn!("Cannot parse config {:?}", files[0]);
                        continue
                    }
                };
                config_path = Some(files[0].clone().into_os_string().into_string().unwrap());
                opt.merge(&cur_conf);
            }else if files.len() > 1 {
                // Log msg that there can be only one current config
                log::warn!("There can be only one current config; More than one config found in {:?}", path);
            }
        }
    }
    (opt.to_config(), config_path)
}

fn get_current_config() -> std::io::Result<(Config, Option<String>)>{
    let buf = std::env::current_dir()?;
    Ok(get_current_config_for_path(buf))
}

fn get_profiles_for_path(mut cur_path: PathBuf) -> HashMap<String, PathBuf> {
    let mut profiles: HashMap<String, PathBuf> = HashMap::new();
    let mut pathes = PathIter::new(cur_path.clone()).collect::<Vec<PathBuf>>();
    cur_path.push(".git");
    pathes = prepend(pathes, cur_path);
    for path in pathes.iter_mut().rev() {
        path.push(".gitconf");
        path.push("profiles");
    }
    for path in pathes.iter_mut().rev() {
        if let Ok(read_dir) = fs::read_dir(path.clone()) {
            let entrys: Vec<std::io::Result<std::fs::DirEntry>> = read_dir.collect();
            for entry in entrys {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    match toml::from_str(
                        match fs::read_to_string(path.clone().into_os_string()){
                            Ok(s) => s,
                            Err(_) => {
                                log::warn!("Cannot read config {:?}", path);
                                continue
                            }
                        }.as_str()
                    ){
                        Ok(cur_conf) => cur_conf,
                        Err(_) => {
                            log::warn!("Cannot parse config {:?}", path);
                            continue
                        }
                    };
                    //log::info!("Succsess parse config {:?}", path);
                    profiles.insert(path.file_name().unwrap().to_str().unwrap().to_string(), path);
                }
            }
        }
    }
    profiles
}

fn get_current_profiles() -> std::io::Result<HashMap<String, PathBuf>>{
    let buf = std::env::current_dir()?;
    Ok(get_profiles_for_path(buf))
}

fn set_profile(src: PathBuf, dst: PathBuf) -> bool {
    let mut dst = {
        let mut git = dst.clone();
        git.push(".git");
        if git.exists() {
            git.push(".gitconf");
            git.push("current");
            git
        }else{
            let mut dst = dst.clone();
            dst.push(".gitconf");
            dst.push("current");
            dst
        }
    };
    if dst.exists() {
        if dst.is_dir() {
            if let Err(e) = std::fs::remove_dir_all(dst.clone()) {
                log::error!("Cannot set profile {:?}", e);
                return false
            }
        }else{
            if let Err(e) = std::fs::remove_file(dst.clone()) {
                log::error!("Cannot set profile {:?}", e);
                return false
            }
        }
    }
    if let Err(e) = std::fs::create_dir_all(dst.clone()) {
        log::error!("Cannot set profile {:?}", e);
        return false
    }
    dst.push(src.file_name().unwrap());
    if let Err(e) = std::fs::copy(src, dst) {
        log::error!("Cannot set profile {:?}", e);
        return false
    }
    return true
}

fn main() {
    simplelog::TermLogger::init(
        simplelog::LevelFilter::Debug,
        simplelog::ConfigBuilder::new().set_time_format_str("").build(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto
    ).unwrap();
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "show-profiles" => {
                let profiles = match get_current_profiles() {
                    Ok(profiles) => profiles,
                    Err(e) => {
                        log::error!("Cannot get available profiles : {:?}", e);
                        return
                    }
                };
                log::info!("Available profiles:");
                for (name, path) in profiles.into_iter() {
                    println!("\t {} at {:?}", name, path);
                }
                return
            }
            "show-profile" => {
                let (config, path) = match get_current_config() {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("Cannot get current profile : {:?}", e);
                        return
                    }
                };
                if let Some(path) = path {
                    let buf = PathBuf::from(path.clone());
                    let name = buf.file_name().unwrap().to_str().unwrap();
                    log::info!("Current profile \"{}\" at {}", name, path);
                }else{
                    log::info!("Current profile is default");
                }
                for line in format!("{:#?}", config).lines() {
                    println!("\t {}", line);
                }
                return
            }
            "set-profile" => {
                let (config, _) = match get_current_config() {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("Cannot get current profile : {:?}", e);
                        return
                    }
                };
                let profiles = match get_current_profiles() {
                    Ok(profiles) => profiles,
                    Err(e) => {
                        log::error!("Cannot get available profiles : {:?}", e);
                        return
                    }
                };
                if args.len() < 3 {
                    if !config.interactive {
                        log::error!("Profile not selected");
                        return
                    }
                    // <TODO> Add interactive profile choise
                    log::error!("Profile not selected");
                    log::info!("Interactive mode is not available yet");
                    return
                }
                let path = match profiles.get(&args[2]) {
                    Some(path) => path.clone(),
                    None => {
                        log::error!("Could not find a profile with name \"{}\"", args[2]);
                        return
                    }
                };
                let cur = match std::env::current_dir() {
                    Ok(cur) => cur,
                    Err(e) => {
                        log::error!("Could not set profile {:?}", e);
                        return
                    }
                };
                if set_profile(path.clone(), cur) {
                    if get_current_config().unwrap().0.apply() {
                        log::info!("Profile \"{}\" has been successfully set from {:?}",
                                        args[2],
                                        path
                        );
                    }
                }
            }
            "set-profile-path" => {
                if args.len() < 3 {
                    log::error!("Profile not selected");
                    return
                }
                let path = PathBuf::from(args[2].clone());
                let cur = match std::env::current_dir() {
                    Ok(cur) => cur,
                    Err(e) => {
                        log::error!("Could not set profile {:?}", e);
                        return
                    }
                };
                if set_profile(path.clone(), cur) {
                    if get_current_config().unwrap().0.apply() {
                        log::info!("Profile \"{}\" has been successfully set from {:?}",
                                        args[2],
                                        path
                        );
                    }
                }
            }
            _ => {
                // Regular git command
                let git = match which("git") {
                    Ok(git) => git.into_os_string().into_string().unwrap(),
                    Err(e) => {
                        log::error!("Cannot find git command : {:?}", e);
                        return
                    }
                };
                // <TODO> Add interactive profile selection when first use gitconf in repo
                let (config, path) = match get_current_config() {
                    Ok(v) => v,
                    Err(e) => {
                        log::error!("Cannot get current profile : {:?}", e);
                        return
                    }
                };
                if !config.apply() { return }
                if config.show_current_profile {
                    if let Some(path) = path {
                        let buf = PathBuf::from(path.clone());
                        let name = buf.file_name().unwrap().to_str().unwrap();
                        log::info!("Current profile \"{}\" at {}", name, path);
                    }else{
                        log::info!("Current profile is default");
                    }
                }
                let mut command = Command::new(git);
                for arg in args[1..].iter() {
                    command.arg(arg);
                }
                let err = command.exec();
                log::error!("Cannot run git command {:?}", err);
            }
        }
    }
}
