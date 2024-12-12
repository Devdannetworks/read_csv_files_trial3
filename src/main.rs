use csv;
use std::error::Error;
use std::{env, fs};

fn read_csv(path: &str, query: &str) -> Result<(), Box<dyn Error>> {
    let file = fs::File::open(path)?;
    let mut reader = csv::Reader::from_reader(file);
    println!("{:?}", reader);

    for e in reader.records() {
        let result = e.unwrap();
        let words_in_result = result.iter().collect::<Vec<_>>().join(", ");

        if words_in_result.contains(query) {
            println!(" {:?}", result);
        } else {
            continue;
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        return println!("Less arguments than expected passed");
    } else {
        let file_path = &args[1];
        let query = &args[2];

        if let Err(e) = read_csv(&file_path, &query) {
            println!("Error reading the file: {}", e);
        }
    }
}
