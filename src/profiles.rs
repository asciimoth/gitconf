use crate::cfg::Config;
use crate::cfg::OptionConfig;
use crate::pth::PathIter;

use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

fn prepend<T>(v: Vec<T>, s: T) -> Vec<T>
where
    T: Clone,
{
    let s = vec![s];
    let mut tmp: Vec<_> = s.to_owned();
    tmp.extend(v);
    tmp
}

pub fn get_current_config_for_path(mut cur_path: PathBuf) -> (Config, Option<String>){
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
                    Err(e) => {
                        log::warn!("Cannot parse config {:?} {:?}", files[0], e);
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

pub fn get_current_config() -> std::io::Result<(Config, Option<String>)>{
    let buf = std::env::current_dir()?;
    Ok(get_current_config_for_path(buf))
}

pub fn get_profiles_for_path(mut cur_path: PathBuf) -> HashMap<String, PathBuf> {
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
                    let _: OptionConfig =  match toml::from_str(
                        match fs::read_to_string(path.clone().into_os_string()){
                            Ok(s) => s,
                            Err(_) => {
                                log::warn!("Cannot read config {:?}", path);
                                continue
                            }
                        }.as_str()
                    ){
                        Ok(cur_conf) => cur_conf,
                        Err(e) => {
                            log::warn!("Cannot parse config {:?} {:?}", path, e);
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

pub fn get_current_profiles() -> std::io::Result<HashMap<String, PathBuf>>{
    let buf = std::env::current_dir()?;
    Ok(get_profiles_for_path(buf))
}

pub fn set_profile(src: PathBuf, dst: PathBuf) -> bool {
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
