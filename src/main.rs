use std::fs::File;
use std::io::{self, BufRead, Read, stdin};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Jakub Stibůrek", version = "1.0", about = "wc util clone", long_about = None)]
struct CCWCArgs {
    #[arg(short, help = "Print the count of bytes in the file")]
    count_bytes: bool,

    #[arg(short, help = "Print the count of lines in the file")]
    lines: bool,

    #[arg(short, help = "Print the count of words in the file")]
    words: bool,

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

    if args.lines {
        let lines_count = count_lines_in_file(&mut file)?;
        result.push_str(&format!(" {}", lines_count).as_str());
    }

    if args.words {
        let words_count = count_words_in_file(&mut file)?;
        result.push_str(&format!(" {}", words_count).as_str());
    }

    println!("{} {}", result, file_location);
    Ok(())
}

fn count_bytes_in_file(file: &mut File) -> io::Result<usize> {
    let mut buffer = Vec::new();
    let bytes_count = file.read_to_end(&mut buffer)?;

    Ok(bytes_count)
}

fn count_lines_in_file(file: &mut File) -> io::Result<usize> {
    let mut lines_count = 0;

    for _ in io::BufReader::new(file).lines() {
        lines_count += 1;
    }

    Ok(lines_count)
}

fn count_words_in_file(file: &mut File) -> io::Result<usize> {
    let mut words_count = 0;

    for line in io::BufReader::new(file).lines() {
        let line = line?;
        let words = line.split_whitespace();
        words_count += words.count();
    }

    Ok(words_count)
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

    #[test]
    fn it_should_count_lines_in_a_file() {
        let mut file = std::fs::File::open("./test.txt").unwrap();

        let lines_count = crate::count_lines_in_file(&mut file).unwrap();

        assert_eq!(lines_count, 7145);
    }

    #[test]
    fn it_should_count_words_in_a_file() {
        let mut file = std::fs::File::open("./test.txt").unwrap();

        let words_count = crate::count_words_in_file(&mut file).unwrap();

        assert_eq!(words_count, 58164);
    }
}
