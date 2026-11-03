use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;

#[derive(Serialize, Deserialize, Debug)]
struct Information {
    article: HashMap<String, String>,
}

pub fn jschanges(current: &str) -> String {
    let mut dbase: Vec<Information> =
        serde_json::from_str(current).unwrap_or_default();

    loop {
        println!("add info - enter 1, read info - enter 2, exit - enter 3");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => {
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                let mut inf = String::new();
                io::stdin().read_line(&mut inf).unwrap();

                
                
                dbase.push(Information {
                    article: HashMap::from([(
                        name.trim().to_string(),
                        inf.trim().to_string(),
                    )]),
                });
            
            
            }
            "2" => {
                if dbase.is_empty() {
                    println!("None info");
                } else {
                    println!("{:?}", dbase);
                }
            }
            "3" => break,
            _ => println!("unknown"),
        }
    }

    serde_json::to_string_pretty(&dbase).unwrap()
}