use encoding_rs::SHIFT_JIS;
use std::{
    io::Write,
    path::{self, PathBuf},
};
use text_io::read;

pub fn convert(input: &PathBuf, output: &PathBuf, is_remove_addressline: bool) {
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

pub fn text_read_file(path: &path::Path) -> String {
    // テキストファイルの内容を一括で読み込む
    let text = std::fs::read(path).expect("入力ファイルを読み込めません");
    let (text, _, _) = SHIFT_JIS.decode(&text);

    text.into_owned()
}

// 16進数文字列を16進数に変換する関数
pub fn convert_to_hex(vec: &Vec<String>) -> Vec<u8> {
    vec.iter()
        .map(|x| u8::from_str_radix(x, 16).expect("cannot convert to hex"))
        .collect::<Vec<u8>>()
}

// 16進数文字列を10進数に変換する関数
pub fn convert_to_dec_string(hex_vec: &Vec<u8>) -> String {
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
    assert!(text_trim("rr").is_empty());
    assert!(text_trim("hrtyhs").is_empty());
    assert!(text_trim("ac").len() == 1);
}

// 16進数ではない部分を除去する関数
pub fn text_trim(text: &str) -> Vec<String> {
    // textを空白区切りでベクタに格納する
    let mut vec: Vec<&str> = text.split_whitespace().collect();
    // 長さが2ではない要素を除去する
    vec.retain(|x| x.len() == 2);

    // 各要素が16進数に変換する
    let mut hex_vec = vec![];
    for x in vec.into_iter() {
        if u8::from_str_radix(x, 16).is_ok() {
            hex_vec.push(x.to_string());
        }
    }

    hex_vec
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

pub fn convert_file_hex_into_dec(hex_vec: &Vec<String>) {
    let hex_vec = convert_to_hex(hex_vec);
    let hex_vec_string = convert_to_dec_string(&hex_vec);
    println!("{}", hex_vec_string);
}

pub fn convert_string_hex_to_dec(input_file: &mut PathBuf, output_file: &mut PathBuf) {
    input_file.set_extension("txt");
    output_file.set_extension("txt");
    let is_remove_addressline = check_remove_addressline();

    convert(input_file, output_file, is_remove_addressline);
    println!("file convert complete!");
}
