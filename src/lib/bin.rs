use crate::errors::Errors;
use std::{fs, path, process};
#[derive(Eq, Hash, Debug, Clone, PartialEq)]
pub struct Bin {
    filepath: String,
    name: String,
    exec: String,
}

impl Bin {
    pub fn from_only_filepath(filepath: &str) -> Self {
        let name = filepath.split('/').last().unwrap().into();
        Bin {
            filepath: filepath.into(),
            name,
            exec: filepath.into(),
        }
    }
    pub fn new(filepath: &str, name: &str, exec: &str) -> Self {
        Bin {
            filepath: filepath.into(),
            name: name.into(),
            exec: exec.into(),
        }
    }
    pub fn filepath(&self) -> &str {
        &self.filepath
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn exec(&self) -> Result<process::Child, Errors> {
        let output = Ok(process::Command::new("sh")
            .args(&["-c", &self.exec])
            .spawn()?);

        //dirty hack to make apps that have a startup time of some sort run.
        std::thread::sleep(std::time::Duration::new(0, 500_000_000));

        output
    }
    pub fn exec_cmd(&self) -> &str {
        &self.exec
    }
}

pub fn get_bins() -> Vec<Bin> {
    let base_dirs = xdg::BaseDirectories::new().unwrap_or_else(|e| {
        eprintln!(
            "This system does not follow XDG spec, aborting, error code for debug:{}",
            e
        );
        process::exit(1);
    });
    let paths = base_dirs.get_data_dirs();
    let mut bins = vec![];
    bins.extend(search_dirs_with_appended_name(paths.clone(), "applications").into_iter());
    bins.extend(search_dirs_with_appended_name(paths, "desktop-directories").into_iter());
    bins
}

fn search_dirs_with_appended_name(paths: Vec<path::PathBuf>, name: &str) -> Vec<Bin> {
    let mut bins = vec![];
    for mut path in paths.into_iter() {
        path.push(path::Path::new(name));
        eprintln!("{:?}", &path);
        let dir_entries = match fs::read_dir(path) {
            Ok(entries) => entries,
            Err(e) => {
                eprintln!("got err, {}, skipping", e);
                continue;
            }
        };
        for item in dir_entries {
            let item = match item {
                Ok(item) => item,
                Err(e) => {
                    eprintln!("got err, {}", e);
                    continue;
                }
            };
            if fs::metadata(item.path()).unwrap().is_file() {
                let bin = match parse_desktop_file_for_bin(&item.path()) {
                    Ok(bin) => bin,
                    Err(e) => {
                        eprintln!("got err, {}, skipping file", e);
                        continue;
                    }
                };
                bins.push(bin);
            }
        }
    }
    bins
}

//I should really make an actual parser instead of this fucking mess.
//I'm sorry whoever reads this.
//Please fix me one day.
//This is painful.
//Just make a dedicated parser please for the love of god.
fn parse_desktop_file_for_bin(path: &path::PathBuf) -> std::result::Result<Bin, Errors> {
    let desktop_file_contents = fs::read_to_string(path)?;
    let (remaining, _) = nom::bytes::complete::take_until::<_, _, ()>("[Desktop Entry]")(
        desktop_file_contents.as_str(),
    )
    .map_err(|_| Errors::BadName)?;
    let (remaining, _) =
        nom::bytes::complete::tag::<_, _, ()>("[Desktop Entry]")(remaining).unwrap();
    let (_, contents) = nom::bytes::complete::take_until::<_, _, (&str, nom::error::ErrorKind)>(
        "[Desktop",
    )(remaining)
    .unwrap_or_else(|e| match e {
        nom::Err::Error((all, _)) => ("", all),
        _ => unreachable!(),
    });
    let mut name = None;
    let mut exec = None;
    for line in contents.split('\n') {
        let (remaining, key) = match nom::bytes::complete::take_until::<_, _, ()>("=")(line) {
            Ok((k, v)) => (k, v),
            _ => continue,
        };
        let (value, _) = match nom::bytes::complete::tag::<_, _, ()>("=")(remaining)
            .map_err(|_| Errors::BadName)
        {
            Ok(v) => v,
            _ => continue,
        };
        if key.trim() == "Name" {
            name = Some(value);
        }
        if key.trim() == "Exec" {
            let value = value
                .split(' ')
                .filter(|c| {
                    ![
                        "%f", "%F", "%u", "%U", "%d", "%D", "%n", "%N", "%i", "%c", "%k", "%v",
                        "%m",
                    ]
                    .iter()
                    .any(|item| c == item)
                })
                .collect::<String>();
            exec = Some(value);
        }
    }
    Ok(Bin::new(
        &path.clone().into_os_string().into_string()?,
        &name.ok_or(Errors::BadName)?,
        &exec.ok_or(Errors::BadExec)?,
    ))
}
