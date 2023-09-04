use apache_avro::{from_value, Reader};
use std::fs;
mod schema;
use schema::Alert;

fn main() {
    const BASE_PATH: &str = "./data/ztf_tmp";
    for path in fs::read_dir(BASE_PATH).unwrap() {
        let path = path.unwrap();
        if path.path().extension().unwrap() == "avro" {
            println!("File {}", path.path().display());
            let content = fs::OpenOptions::new().read(true).open(path.path()).unwrap();
            let reader = Reader::new(&content).unwrap();
            for value in reader {
                match value {
                    Ok(value) => match from_value::<Alert>(&value) {
                        Ok(alert) => println!("{}", alert.objectId),
                        Err(error) => println!("Error: {}", error),
                    },
                    Err(error) => println!("Error: {}", error),
                }
            }
        }
    }
}
