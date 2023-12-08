use std::fs::File;
use std::io::{self, Read, stdin};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Jakub Stibůrek", version = "1.0", about = "wc util clone", long_about = None)]
struct CCWCArgs {
    #[arg(short)]
    count_bytes: bool,

    #[arg(required = true)]
    file_path: Option<std::path::PathBuf>,
}

fn main() -> io::Result<()> {
    let mut result = String::new();
    let args = CCWCArgs::parse();
    let mut file_location = String::new();

    if let Some(path) = args.file_path {
        if let Some(path_as_string) = path.to_str() {
            file_location = path_as_string.to_string();
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Invalid file path"))?;
        }
    } else {
        stdin().read_line(&mut file_location)?;
        Err(io::Error::new(io::ErrorKind::Other, "No file path provided"))?;
    }

    let mut file = File::open(file_location.trim())?;

    if args.count_bytes {
        let bytes_count = count_bytes_in_file(&mut file)?;
        result.push_str(&bytes_count.to_string());
    }

    println!("{} {}", result, file_location);
    Ok(())
}

fn count_bytes_in_file(file: &mut File) -> io::Result<usize> {
    let mut buffer = Vec::new();
    let bytes_count = file.read_to_end(&mut buffer)?;

    Ok(bytes_count)
}

#[cfg(test)]
mod tests {
    use crate::count_bytes_in_file;

    #[test]
    fn it_should_count_bytes_in_a_file() {
        let mut file = std::fs::File::open("./test.txt").unwrap();

        let bytes_count = count_bytes_in_file(&mut file).unwrap();

        assert_eq!(bytes_count, 342190);
    }
}
