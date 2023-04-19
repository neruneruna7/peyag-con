use clap::{Parser, Subcommand};
use encoding_rs::SHIFT_JIS;
use std::{
    io::Write,
    path::{self, PathBuf},
};
use text_io::read;

#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    #[clap(name = "f")]
    FileCon {
        #[arg(short = 'i', long)]
        input_file: std::path::PathBuf,
        #[arg(short = 'o', long)]
        output_file: std::path::PathBuf,
    },

    #[clap(name = "d")]
    DecCon { hex_vec: Vec<String> },
}

fn is_hex_vec(vec: &Vec<String>) -> bool {
    for x in vec.iter() {
        if let Ok(_) = u8::from_str_radix(x, 16) {
            continue;
        } else {
            return false;
        }
    }
    true
}

// is_hex_vecのテスト
#[test]
fn test_is_hex_vec() {
    assert!(is_hex_vec(&vec!["00".to_string(), "ff".to_string()]));
    assert!(!is_hex_vec(&vec![
        "00".to_string(),
        "ff".to_string(),
        "gg".to_string()
    ]));
    assert!(!is_hex_vec(&vec![
        "00".to_string(),
        "ff".to_string(),
        "00".to_string()
    ]));
}

// 16進数ではない部分を除去する関数
fn text_trim(text: &str) -> Vec<String> {
    // textを空白区切りでベクタに格納する
    let mut vec: Vec<&str> = text.split_whitespace().collect();
    // 長さが2ではない要素を除去する
    vec.retain(|x| x.len() == 2);

    // 各要素が16進数に変換する
    let mut hex_vec = vec![];
    for x in vec.into_iter() {
        if let Ok(_) = u8::from_str_radix(x, 16) {
            hex_vec.push(x.to_string());
        }
    }

    hex_vec
}

// 16進数文字列を16進数に変換する関数
fn convert_to_hex(vec: &Vec<String>) -> Vec<u8> {
    vec.iter()
        .map(|x| u8::from_str_radix(x, 16).unwrap())
        .collect::<Vec<u8>>()
}

// 16進数文字列を10進数に変換する関数
fn convert_to_dec_string(hex_vec: &Vec<u8>) -> String {
    let hex_vec = hex_vec
        .iter()
        .map(|x| format!("{}", x))
        .collect::<Vec<String>>();

    hex_vec.join(" ")
}
// text_trimのテスト
#[test]
fn test_text_trim() {
    assert!(text_trim("00").len() == 1);
    assert!(text_trim("rr").len() == 0);
    assert!(text_trim("hrtyhs").len() == 0);
    assert!(text_trim("ac").len() == 1);
}

fn text_read_file(path: &path::Path) -> String {
    // テキストファイルの内容を一括で読み込む
    let text = std::fs::read(path).expect("入力ファイルを読み込めません");
    let (text, _, _) = SHIFT_JIS.decode(&text);
    let text = text.into_owned();

    text
}

fn check_remove_addressline() -> bool {
    print!("最初のADDRESSの行を削除しますか？ (y/n): ");

    loop {
        let yn: char = read!();
        if yn == 'y' || yn == 'n' {
            println!();
            break yn == 'y';
        } else {
            println!();
            print!("yかnを入力してください: ");
        }
    }
}

/*
fn input_search_hex() -> Option<Vec<String>> {
    print!("検索したい16進数文字があれば入力して下さい: ");
    let search_hex: String = read!();
    let search_hex = search_hex.trim().split_whitespace().collect::<Vec<&str>>();
    let search_hex = if search_hex.len() == 0 {
        None
    } else {
        Some(search_hex.iter().map(|x| x.to_string()).collect())
    };
    search_hex
}
*/

fn convert(input: &PathBuf, output: &PathBuf, is_remove_addressline: bool) {
    // 出力先のファイルを指定
    let mut output = std::fs::File::create(output).expect("出力ファイルを作成できません");

    // 破棄された処理 let is_remove_addressline = check_remove_addressline();

    // テキストファイルの内容を一括で読み込む
    let text = text_read_file(input);

    // 16進数ではない部分を除去し，ベクタに格納する
    let mut vec = text_trim(&text);

    if is_remove_addressline {
        // 初めの16番目までの要素を削除する
        // つまり，最初のADDRESSの行を削除する
        vec.drain(..16);
    }

    // ベクタの各要素を16進数に変換する
    let hex_vec = convert_to_hex(&vec);

    // ベクタの各要素を文字列に再変換する
    let text = convert_to_dec_string(&hex_vec);

    // ファイルに書き込む
    output
        .write(text.as_bytes())
        .expect("出力ファイルに書き込めません");
}

fn main() {
    let args = Args::parse();
    match args.subcommand {
        SubCommand::FileCon {
            mut input_file,
            mut output_file,
        } => {
            input_file.set_extension("txt");
            output_file.set_extension("txt");
            let is_remove_addressline = check_remove_addressline();

            convert(&input_file, &output_file, is_remove_addressline);
            println!("file convert complete!");
        }
        SubCommand::DecCon { hex_vec } => {
            let hex_vec = convert_to_hex(&hex_vec);
            let hex_vec_string = convert_to_dec_string(&hex_vec);
            println!("{}", hex_vec_string);
        }
    }

    /*/
    // 1つめの要素はプログラム名なので削除する
    args.drain(..1);

    if is_hex_vec(&args) {
        let hex_vec = convert_to_hex(&args);
        let hex_vec_string = convert_to_dec_string(&hex_vec);
        println!("{}", hex_vec_string);
    } else {
        if &args.len() != &3 {
            eprintln!("引数が不正です");
            eprintln!("引数1: 入力ファイルパス, 引数2: --y or --n, 引数3: 出力ファイルパス");
            eprintln!("引数2は最初のADDRESSの行を削除するかどうかを指定します");
            eprintln!("ファイルパスの拡張子を含める必要はありません");
            eprintln!("強制的にtxtに変換されます");
            return;
        }
        let mut input_path = PathBuf::from(&args[0]);
        let mut output_path = PathBuf::from(&args[2]);

        // ファイルパスの拡張子をtxtにする
        input_path.set_extension("txt");
        output_path.set_extension("txt");

        let is_remove_addressline = args[1] == "--y";

        convert(&input_path, &output_path, is_remove_addressline);
        println!("file convert complete!");
    }
    */
}
