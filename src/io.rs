use std::fs::File;
use std::io::{self, BufRead, Error, Seek, SeekFrom, Write};

pub fn input(prompt: &str) -> String {
    print!("{}", prompt.to_string());
    io::stdout().flush().unwrap();
    let mut result = String::new();
    io::stdin().read_line(&mut result).ok();
    return result.trim().parse().ok().unwrap();
}

/// ファイルを開く
pub fn open_file(name: String) -> Result<File, Error> {
    let mut binding = std::fs::OpenOptions::new();
    let options = binding.read(true).write(true);
    options.open(name)
}

pub fn read_specific_line(mut file: &File, line_number: usize) -> io::Result<String> {
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
    let reader = io::BufReader::new(file);

    let lines: Vec<_> = reader.lines().collect::<io::Result<_>>()?;
    // dbg!(line_number, lines.len());
    if line_number > lines.len() {
        panic!("Line number out of range");
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

    // dbg!(contents.clone());

    // dbg!("test line", line!(), file.metadata()?);
    file.set_len(0)?;
    // dbg!("test line", line!(), file.metadata()?);
    file.seek(SeekFrom::Start(0))?;
    // dbg!("test line", line!(), file.metadata()?);
    file.write_all(contents.as_bytes())?;
    // dbg!("test line", line!(), file.metadata()?);
    file.sync_all()?; // ファイルをディスクに書き込む
                      // dbg!("test line", line!(), file.metadata()?);

    Ok(())
}

#[cfg(test)]
mod test_file {
    use std::fs::{self, File};
    use std::io::{self, BufRead};

    use crate::io::{open_file, write_specific_line};
    #[test]
    fn test_write_text() {
        // // テスト用の一時ファイルを作成
        let path = r"test_file.txt";
        // let mut file = File::create(path).expect("Could not create file");

        // // テスト用のデータをファイルに書き込む
        // file.write_all(b"Line 1\nLine 2\nLine 3\r\n")
        //     .expect("Could not write to file");

        // ファイルを読み書きモードで開く
        let mut file = open_file(path.to_string()).expect("ファイルが開けませんでした");

        // 特定の行にテキストを書き込む
        let text_to_write = "This is a test line.";
        write_specific_line(&mut file, 2, text_to_write).expect("Failed to write specific line");

        // ファイルを読み込んで、変更が反映されていることを確認する
        let lines = io::BufReader::new(File::open(path).expect("Could not open file"))
            .lines()
            .map(|l| l.expect("Could not read line"))
            .collect::<Vec<String>>();

        // 変更が反映されていることを確認
        assert_eq!(lines[1], text_to_write);

        // テストが終了したらファイルを削除する
        fs::remove_file(path).expect("Failed to remove file");
    }
}
