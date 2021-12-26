use serde::{Deserialize, Serialize};
use std::io::{Write};
use std::process::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Module {
    #[serde(rename = "pacman")]
    Pacman { packages: Vec<String> },
    #[serde(rename = "aur")]
    Aur { packages: Vec<String> },
    #[serde(rename = "script")]
    Script { lines: Vec<String> },
}

impl Module {
    pub fn execute(&self) -> Result<(), Vec<String>> {
        match self {
            Self::Pacman { packages } => {
                println!("Executing Pacman");
                let mut errors: Vec<String> = vec![];
                let args = vec![
                    "pacman".to_string(),
                    "--noconfirm".to_string(),
                    "-S".to_string(),
                    "--needed".to_string(),
                    "-".to_string(),
                ];
                let child_result = Command::new("/sbin/sudo")
                    .args(args)
                    .stdin(Stdio::piped())
                    .spawn();
                if let Ok(mut child) = child_result {
                    {
                        let mut stdin = child.stdin.as_ref().unwrap();
                        if let Err(err) = stdin.write_all((packages.join("\n")).as_bytes()) {
                            errors
                                .push(format!("error while writing package list to child: {}", err).to_string());
                        }
                    }
                    let status = child.wait().expect("Command wasn't running");
                    if !status.success() {
                        errors.push(
                                match status.code() {
                                    Some(code) => format!("Exited with status code: {}", code).to_string(),
                                    None       => "Process terminated by signal".to_string()
                                }
                        );
                    }
                    if errors.len() == 0 {
                        Ok(())
                    } else {
                        Err(errors)
                    }
                } else {
                    Err(vec![child_result.unwrap_err().to_string()])
                }
            }
            Self::Aur { packages } => {
                println!("Executing AUR");
                let mut errors: Vec<String> = vec![];
                let args = vec![
                    "paru".to_string(),
                    "--noconfirm".to_string(),
                    "-S".to_string(),
                    "--needed".to_string(),
                    "-".to_string(),
                ];
                let child_result = Command::new("/bin/env")
                    .args(args)
                    .stdin(Stdio::piped())
                    .spawn();
                if let Ok(mut child) = child_result {
                    {
                        let mut stdin = child.stdin.as_ref().unwrap();
                        if let Err(err) = stdin.write_all((packages.join("\n")).as_bytes()) {
                            errors
                                .push(format!("error while writing package list to child: {}", err).to_string());
                        }
                    }
                    let status = child.wait().expect("Command wasn't running");
                    if !status.success() {
                        errors.push(
                                match status.code() {
                                    Some(code) => format!("Exited with status code: {}", code).to_string(),
                                    None       => "Process terminated by signal".to_string()
                                }
                        );
                    }
                    if errors.len() == 0 {
                        Ok(())
                    } else {
                        Err(errors)
                    }
                } else {
                    Err(vec![child_result.unwrap_err().to_string()])
                }
            }
            Self::Script { lines } => {
                let mut args = vec!["-c".to_string()];
                args.extend(lines.clone());
                println!("Executing config script");
                if let Err(x) = Command::new("/bin/bash").args(args).status() {
                    Err(vec![x.to_string()])
                } else {
                    Ok(())
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    // modules: HashMap<String, Module>,
    pub modules: Vec<Module>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub packages: Vec<Package>,
}
