use std::{
    env::args,
    fs::File,
    io::{BufRead, BufReader, Stdin, stdin},
};

fn main() {
    // 引数を受け取る
    let args: Vec<String> = args().collect();

    if args.len() > 1 {
        // ファイルを read する処理
        let file: File = match File::open(&args[1]) {
            Ok(f) => f,
            Err(e) => panic!("{e}")
        };
        let br: BufReader<File> = BufReader::new(file);
        read_line(br);
    } else {
        // 標準出力を read する処理
        let stdin: Stdin = stdin();
        let br: BufReader<Stdin> = BufReader::new(stdin);
        read_line(br);
    }
}

/// ファイル・標準出力の内容を、行数ともに表示する関数。
fn read_line<T: BufRead>(br: T) {
    for (i, result) in br.lines().enumerate() {
        match result {
            Ok(line) => println!("{}: {}", i+1, line),
            Err(e) => panic!("{e}")
        }
    }
}