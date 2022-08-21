use clap::{App, Arg, ArgAction};
use std::{fs, io};
use std::fmt::Display;
use std::io::{Write};
use std::path::{PathBuf};
use std::process::{Command};

const TMP_DIR: &str = "/tmp";
const APP_DIR: &str = ".mktpl";

fn main() {
    let matches = App::new("mktpl")
        .arg(Arg::with_name("template_name")
            .required_unless_present("list"))
        .arg(Arg::with_name("file_path")
            .default_value(".")
            .requires("template_name"))
        .arg(Arg::with_name("list")
            .exclusive(true)
            .long("list")
            .short('l')
            .action(ArgAction::SetTrue))
        .arg(Arg::with_name("yes")
            .short('y')
            .takes_value(false)
            .action(ArgAction::SetTrue)
        )
        .get_matches();

    let mut config_dir;
    if let Some(home_dir) = dirs::home_dir() {
        config_dir = home_dir;
        config_dir.push(APP_DIR);
    } else {
        print_error("Could not find the home directory.");
        return;
    }


    if let Some(template_name) = matches.get_one::<String>("template_name") {
        let template_list = get_template_list(&config_dir);
        match template_list {
            Err(err) => print_error(err),
            Ok(vec) => {
                if vec.contains(template_name) {
                    let file_path = matches.get_one::<String>("file_path");
                    let _ = copy_template(config_dir, template_name, file_path.unwrap());
                } else {
                    let fork_result;
                    if let Some(x) = matches.get_one::<bool>("yes") {
                        if *x {
                            fork_result = new_template(template_name, config_dir);
                        } else {
                            let line = ask(format!("No template called {}, would you like to create it?", template_name).as_str(), "(y/N)");
                            // Trim ending newline
                            fork_result = match line.trim().to_lowercase().as_str() {
                                "y" => {
                                    println!("Here");
                                    new_template(template_name, config_dir)
                                }
                                x => {
                                    println!("Shouldnt be Here {}", "y" == x);
                                    Ok(())
                                }
                            };
                        }

                        match fork_result {
                            Err(err) => print_error(err),
                            Ok(_) => {}
                        }
                    }
                }
            }
        }
    } else if let Some(_) = matches.get_one::<bool>("list") {
        match list_templates(config_dir) {
            Err(err) => print_error(err),
            _ => {}
        }
    }
}

fn list_templates(config_dir: PathBuf) -> Result<(), io::Error> {
    if !config_dir.exists() {
        std::fs::create_dir(&config_dir)?;
    }
    let template_list: Vec<String> = get_template_list(&config_dir)?;
    if template_list.is_empty() {
        println!("No templates");
    } else {
        for file_name in template_list {
            println!("{}", file_name);
        }
    }
    Ok(())
}

fn get_template_list(path_buf: &PathBuf) -> Result<Vec<String>, io::Error> {
    let file = fs::read_dir(path_buf)?;
    Ok(file
        .into_iter()
        .filter(|entry| entry.is_ok())
        .map(|entry| entry.unwrap().path())
        .filter(|entry| !entry.is_dir())
        .map(|path_buf|
            String::from(path_buf
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()))
        .collect())
}

fn ask(question: &str, prefix: &str) -> String {
    println!("{}", question);
    print!("{} ", prefix);
    let _ = io::stdout().flush();
    let mut line = String::new();
    let _ = std::io::stdin().read_line(&mut line);
    line
}

fn copy_template(config_dir: PathBuf, template_name: &String, file_path: &String) -> Result<(), io::Error> {
    let mut src_path = config_dir;
    let mut dest_path = PathBuf::from(file_path);
    src_path.push(template_name);
    dest_path.push(template_name);

    fs::copy(src_path, dest_path)?;
    Ok(())
}

fn new_template(template_name: &String, app_dir: PathBuf) -> Result<(), io::Error> {
    // Form the tmp dir path with file name + extension for editor syntax highlighting
    let tmp_path: PathBuf = [TMP_DIR, template_name].iter().collect();
    let mut dest_dir = app_dir;
    dest_dir.push(template_name);
    // TODO: Make a fallback for systems that don't use the 'editor' command. ($EDITOR or $VISUAL)
    // Spawn 'editor' with the tmp path
    let status = Command::new("editor")
        .arg(tmp_path.as_os_str())
        .spawn()?.wait()?;
    if status.success() {
        println!("Child status: {} dest_dir: {:?}", status, dest_dir);
        fs::copy(&tmp_path, dest_dir)?;
        fs::remove_file(tmp_path)?;
    } else {
        print_error(format!("Editor closed with exit code {}. No files have been updated.", status.code().unwrap()));
    }
    Ok(())
}

fn print_error(error: impl Display) {
    let name = std::env::args().next().unwrap();
    eprintln!("{}: {}", name, error);
    eprintln!("Try '{} --help' for usage information", name);
}