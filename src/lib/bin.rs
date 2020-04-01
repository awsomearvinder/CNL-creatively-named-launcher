use std::{fs, io, path};
use xdg;

use crate::lib::errors::Errors;

#[derive(Eq, Hash, Debug, Clone, PartialEq)]
pub struct Bin {
    filepath: String,
    name: String,
}

impl Bin {
    pub fn from_only_filepath(filepath: &str) -> Self {
        Bin {
            filepath: filepath.into(),
            name: filepath.split("/").last().unwrap().into(),
        }
    }
    pub fn new(filepath: &str, name: &str) -> Self {
        Bin {
            filepath: filepath.into(),
            name: name.into(),
        }
    }
    pub fn filepath(&self) -> &str {
        &self.filepath
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

pub fn get_bins() -> Vec<Bin> {
    let base_dirs = xdg::BaseDirectories::new().unwrap();
    let paths = base_dirs.get_data_dirs();
    let mut bins = vec![];
    bins.extend(search_dirs_with_appended_name(paths.clone(), "applications").into_iter());
    bins.extend(search_dirs_with_appended_name(paths.clone(), "desktop-directories").into_iter());
    bins
}

fn search_dirs_with_appended_name(paths: Vec<path::PathBuf>, name: &str) -> Vec<Bin> {
    let mut bins = vec![];
    eprintln!("{:?}", &paths);
    for mut path in paths.into_iter() {
        path.push(path::Path::new(name));
        let dir_entries = match fs::read_dir(path) {
            Ok(entries) => entries,
            Err(e) => {
                eprintln!("got err, {:?}, skipping", e);
                continue;
            }
        };
        for item in dir_entries {
            //unwrap the item in the dir, otherwise continue.
            //item at this point in time is either a file, directory, or a symlink.
            let item = match item {
                Ok(item) => item,
                Err(_) => continue,
            };
            match item.metadata() {
                Ok(metadata) => {
                    if metadata.is_file() {
                        bins.push(match parse_desktop_file_for_bin(&item.path()) {
                            Ok(bin) => bin,
                            Err(e) => {
                                eprintln!("{}, skipping file", e);
                                continue;
                            }
                        });
                    }
                }
                Err(_) => {
                    eprintln!("found symlink, not traversing.");
                }
            }
        }
    }
    bins
}

fn parse_desktop_file_for_bin(path: &path::PathBuf) -> std::result::Result<Bin, Errors> {
    let desktop_file_contents = fs::read_to_string(path)?;
    let desktop_file_contents = desktop_file_contents.split("\n");
    for key_val in desktop_file_contents {
        eprintln!("{}", &key_val);
        if key_val.split("=").nth(0) == Some("Name") {
            let name = match key_val.split("=").nth(1) {
                Some(val) => val,
                None => return Err(Errors::BadName),
            };
            return Ok(Bin::new(&path.to_str().unwrap(), name));
        }
    }
    Err(Errors::BadName)
}
