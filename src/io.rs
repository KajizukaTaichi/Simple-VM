use std::fs::File;
use std::io::{self, BufRead, Error, Seek, SeekFrom, Write};

pub fn input(prompt: &str) -> String {
    print!("{}", prompt.to_string());
    io::stdout().flush().unwrap();
    let mut result = String::new();
    io::stdin().read_line(&mut result).ok();
    return result.trim().parse().ok().unwrap();
}

/// ファイルを読み込む
pub fn get_file(name: String) -> Result<File, Error> {
    let f = File::open(name.trim())?;
    return Ok(f);
}

pub fn read_specific_line(mut file: &std::fs::File, line_number: usize) -> io::Result<String> {
    file.seek(SeekFrom::Start(0))?;
    let reader = io::BufReader::new(file);

    let lines = reader.lines().enumerate();
    for (index, line) in lines {
        if index == line_number - 1 {
            return line;
        }
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidInput,
        "Line number out of range",
    ))
}

pub fn write_specific_line(mut file: &File, line_number: usize, text: &str) -> io::Result<()> {
    file.seek(SeekFrom::Start(0))?;
    let reader = io::BufReader::new(&mut file);

    let lines: Vec<_> = reader.lines().collect::<io::Result<_>>()?;
    if line_number > lines.len() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Line number out of range",
        ));
    }

    let mut contents = String::new();
    for (index, line) in lines.into_iter().enumerate() {
        if index == line_number - 1 {
            contents.push_str(&text);
            contents.push('\n');
        } else {
            contents.push_str(&line);
            contents.push('\n');
        }
    }

    file.set_len(0)?;
    file.seek(SeekFrom::Start(0))?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}
