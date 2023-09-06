use apache_avro::{from_value, Reader};
use std::fs;
mod schema;
use clap::Parser;
use prettytable::{Cell, Row, Table};
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::{CompletionType, Config, EditMode, Editor};
use schema::Alert;
use std::collections::HashMap;

#[derive(Parser)]
struct Args {
    file_name: String,
}

fn print_row(alert: &Alert) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("ALERT ID"),
        Cell::new("MJD"),
        Cell::new("RA"),
        Cell::new("DEC"),
        Cell::new("MAGLIM"),
        Cell::new("MAG"),
        Cell::new("MAGERR"),
    ]));
    let mut diffmaglim: String = "None".to_string();
    if alert.candidate.diffmaglim.is_some() {
        diffmaglim = alert.candidate.diffmaglim.unwrap().to_string();
    }
    table.add_row(Row::new(vec![
        Cell::new(&alert.objectId),
        Cell::new(&{ &alert.candidate.jd - 2400000.5 }.to_string()),
        Cell::new(&alert.candidate.ra.to_string()),
        Cell::new(&alert.candidate.dec.to_string()),
        Cell::new(&diffmaglim),
        Cell::new(&alert.candidate.magpsf.to_string()),
        Cell::new(&alert.candidate.sigmapsf.to_string()),
    ]));
    table.printstd();
}

fn main() -> rustyline::Result<()> {
    let args = Args::parse();
    let mut alerts = HashMap::new();
    match fs::read_dir(&args.file_name) {
        Ok(directory) => {
            println!("Reading directory {} ...", args.file_name);
            for path in directory {
                match path {
                    Ok(path) => {
                        match path.path().extension() {
                            Some(extension) => {
                                if extension == "avro" {
                                    //println!("File {}", path.path().display());
                                    let content = fs::OpenOptions::new()
                                        .read(true)
                                        .open(path.path())
                                        .unwrap();
                                    match Reader::new(&content) {
                                        Ok(reader) => {
                                            for value in reader {
                                                match value {
                                                    Ok(value) => {
                                                        match from_value::<Alert>(&value) {
                                                            Ok(alert) => {
                                                                // TODO: Add alerts from alert.prv_candidates
                                                                alerts.insert(
                                                                    alert.objectId.clone(),
                                                                    alert,
                                                                );
                                                            }
                                                            Err(error) => {
                                                                println!("Error: {}", error)
                                                            }
                                                        }
                                                    }
                                                    Err(error) => println!("Error: {}", error),
                                                }
                                            }
                                        }
                                        Err(error) => println!("Error: {}", error),
                                    }
                                }
                            }
                            None => (), // When file have no extension
                        }
                    }
                    Err(error) => println!("Error: {}", error),
                }
            }
        }
        Err(error) => println!("Error: {}", error),
    }
    let config = Config::builder()
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Vi)
        .build();
    let mut rl: Editor<(), FileHistory> = Editor::with_config(config)?;
    println!("Enter a ztf alert id to search. This search is case-sensitive. Press CTRL-C or CTRL-D to close this.");
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line.to_lowercase().starts_with("show all") {
                    for (alert_id, _) in &alerts {
                        println!("{}", alert_id)
                    }
                } else if line.starts_with("ZTF") {
                    match alerts.get(&line) {
                        Some(alert) => print_row(alert),
                        None => println!("Alert '{}' not found.", line),
                    }
                } else {
                    println!("Command not found.")
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}
