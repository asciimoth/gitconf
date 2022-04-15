use std::path::PathBuf;
use std::collections::HashMap;
use serde::Deserialize;
use std::fs;

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
            select_profile_on_first_use: self.select_profile_on_first_use.unwrap_or(true),
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

fn get_current_config_for_path(mut cur_path: PathBuf) -> Config{
    let mut opt = OptionConfig::new();
    let mut pathes = PathIter::new(cur_path.clone()).collect::<Vec<PathBuf>>();
    cur_path.push("./git");
    pathes = prepend(pathes, cur_path);
    for path in pathes.iter_mut().rev() {
        path.push(".gitconf");
        path.push("current");
    }
    for path in pathes.iter_mut().rev() {
        //println!("{:?}", path);
        if let Ok(read_dir) = fs::read_dir(path) {
            let entrys: Vec<std::io::Result<std::fs::DirEntry>> = read_dir.collect();
            let mut files: Vec<PathBuf> = Vec::new();
            for entry in entrys {
                if let Ok(entry) = entry {
                    files.push(entry.path())
                }
            }
            if files.len() == 1 {
                //println!("\t{:?}", files[0]);
                let cur_conf: OptionConfig = match toml::from_str(
                    match fs::read_to_string(files[0].clone().into_os_string()){
                        Ok(s) => s,
                        Err(_) => {
                            /* Log warn message */
                            continue
                        }
                    }.as_str()
                ){
                    Ok(cur_conf) => cur_conf,
                    Err(_) => {
                        /* Log warn message */
                        continue
                    }
                };
                //println!("\t\t{:?}", cur_conf);
                opt.merge(&cur_conf);
            }else if files.len() > 1 {
                // Log msg that there can be only one current config
            }
        }
    }
    opt.to_config()
}

fn get_current_config() -> std::io::Result<Config>{
    let buf = std::env::current_dir()?;
    Ok(get_current_config_for_path(buf))
}

fn main() {
    println!("\n{:?}", get_current_config());
    /*for path in PathIter::current().unwrap() {
        println!("{:?}", path);
    }
    for path in PathIter::current().unwrap().collect::<Vec<PathBuf>>().iter().rev() {
        println!("{:?}", path);
    }
    for path in PathIter::new(PathBuf::from("/etc/a/b/c/d")) {
        println!("{:?}", path);
    }*/
}
