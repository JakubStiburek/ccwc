use std::fs::File;
use std::io::{self, BufRead, Read, SeekFrom, stdin, Seek};
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

    #[arg(required = false)]
    file_path: Option<std::path::PathBuf>,
}

fn main() -> io::Result<()> {
    let mut result = String::new();
    let args = CCWCArgs::parse();
    let mut file_location = String::new();
    let mut stdin_text = String::new();
    let mut file: Option<File> = None;

    if let Some(path) = args.file_path {
        if let Some(path_as_string) = path.to_str() {
            file_location = path_as_string.to_string();
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "Invalid file path"))?;
        }

        file = Some(File::open(file_location.trim())?);
    }

    if args.count_bytes {
        if let Some(file) = &mut file {
            let bytes_count = count_bytes_in_file(file)?;
            file.seek(SeekFrom::Start(0))?;
            result.push_str(&bytes_count.to_string());
        } else {
            stdin().read_to_string(&mut stdin_text)?;
            result.push_str(&stdin_text.len().to_string());
        }
    }

    if args.lines {
        if let Some(file) = &mut file {
            let lines_count = count_lines_in_file(file)?;
            file.seek(SeekFrom::Start(0))?;
            result.push_str(&format!(" {}", lines_count).as_str());
        } else {
            stdin().read_to_string(&mut stdin_text)?;
            let lines_count = stdin_text.lines().count();
            result.push_str(&format!(" {}", lines_count).as_str());
        }
    }

    if args.words {
        if let Some(file) = &mut file {
            let words_count = count_words_in_file(file)?;
            file.seek(SeekFrom::Start(0))?;
            result.push_str(&format!(" {}", words_count).as_str());
        } else {
            stdin().read_to_string(&mut stdin_text)?;
            let words_count = stdin_text.split_whitespace().count();
            result.push_str(&format!(" {}", words_count).as_str());
        }
    }

    if !args.count_bytes && !args.lines && !args.words {
        if let Some(file) = &mut file {
            let bytes_count = count_bytes_in_file(file)?;
            let lines_count = count_lines_in_file(file)?;
            let words_count = count_words_in_file(file)?;
            file.seek(SeekFrom::Start(0))?;
            result.push_str(&format!(" {} {} {}", lines_count, words_count, bytes_count).as_str());
        } else {
            stdin().read_to_string(&mut stdin_text)?;
            let bytes_count = stdin_text.len();
            let lines_count = stdin_text.lines().count();
            let words_count = stdin_text.split_whitespace().count();
            result.push_str(&format!(" {} {} {}", lines_count, words_count, bytes_count).as_str());
        }
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
