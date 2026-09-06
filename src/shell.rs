use std::io::{stdout, BufRead, Write};
use std::{fs, io};

pub fn run_shell() {
    //shell::run();
    let mut line = "".to_string();
    loop {
        line = "".to_string();
        print!("> ");
        stdout().flush().expect("Failed to flush stdout");
        let stdin = io::stdin();
        let s = stdin.lock().read_line(&mut line);
        if let Ok(_) = s {
            let len = line.trim_end_matches(&['\r', '\n'][..]).len();
            line.truncate(len);
            //println!("!{}!", line.clone());
            let fin = traite(line.clone());
            if fin {
                break;
            }
        }
    }
}

fn traite(ligne: String) -> bool {
    let ligne2 = ligne.trim_end();
    if ligne2.starts_with("exit") {
        return true;
    } else if ligne2 == ("ls") || ligne2.starts_with("ls ") {
        let ligne3 = ligne2.trim_start_matches("ls");
        command_ls(ligne3.to_string()).expect("TODO: panic message");
    } else if ligne2 == ("echo") || ligne2.starts_with("echo ") {
        let ligne3 = ligne2.trim_start_matches("echo");
        command_echo(ligne3.to_string());
    }
    false
}

fn command_ls(mut ligne: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut repertoire = ".".to_string();
    ligne = ligne.trim().to_string();
    if ligne.len() > 0 {
        repertoire = ligne.clone();
    }
    let mut entries = fs::read_dir(repertoire)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    for entry in entries {
        println!("{}", entry.display());
    }

    Ok(())
}

fn command_echo(mut ligne: String) {
    ligne = ligne.trim().to_string();
    println!("{}", ligne);
}
