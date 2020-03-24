use std::{fs, path};
use xdg;

#[derive(Eq, Hash, Debug, Clone, PartialEq)]
pub struct Bin {
    filepath: String,
}

impl Bin {
    pub fn new(filepath: &str) -> Self {
        Bin {
            filepath: String::from(filepath),
        }
    }
    pub fn filepath(&self) -> &str {
        &self.filepath
    }
    pub fn name(&self) -> &str {
        //name of bin is at end of file-path split by both / and .
        let name_section = self.filepath.split("/").last().unwrap();
        let name_section = name_section.split(".").collect::<Vec<_>>();
        name_section[name_section.len() - 2]
    }
}

pub fn get_bins() -> Vec<Bin> {
    let base_dirs = xdg::BaseDirectories::new().unwrap();
    let paths = base_dirs.get_data_dirs();
    let mut bins = vec![];
    bins.extend(search_dir_with_appended_name(paths.clone(), "applications").into_iter());
    bins.extend(search_dir_with_appended_name(paths.clone(), "desktop-directories").into_iter());
    bins
}

fn search_dir_with_appended_name(paths: Vec<path::PathBuf>, name: &str) -> Vec<Bin> {
    let mut bins = vec![];
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
            let item = match item {
                Ok(item) => item,
                Err(_) => continue,
            };
            match item.metadata() {
                Ok(metadata) => {
                    if metadata.is_file() {
                        bins.push(Bin::new(item.path().to_str().unwrap()));
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
