use clap::{Parser, Subcommand};
use encoding_rs::SHIFT_JIS;
use std::{
    io::Write,
    path::{self, PathBuf},
};
use text_io::read;

mod api;
/// 使い方： winFdumpが吐く，バイナリをテキストにしたファイルから16進数部分を抽出し，10進数に変換します.
// example: peyag-con f -i input.txt -o output.txt
// example: peyag-con d 00 01 02 aa ff
#[derive(Debug, Parser)]
struct Args {
    /// サブコマンドとして f と d があります.
    #[clap(subcommand)]
    subcommand: Option<SubCommand>,
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    /// f: ファイルから16進数部分を抽出し，10進数に変換します.
    #[clap(name = "f")]
    FileCon {
        /// 拡張子を入力する必要はありません. 自動でtxtを付与します
        #[arg(short = 'i', long)]
        input_file: std::path::PathBuf,
        /// 拡張子を入力する必要はありません. 自動でtxtを付与します
        #[arg(short = 'o', long)]
        output_file: std::path::PathBuf,
    },

    /// d: 16進数文字列を10進数に変換します
    #[clap(name = "d")]
    DecCon { hex_vec: Vec<String> },
}

fn main() {
    let args = Args::parse();

    match args.subcommand {
        None => {
            api::convert_cli();
        }
        Some(subcommand) => match subcommand {
            SubCommand::FileCon {
                mut input_file,
                mut output_file,
            } => api::convert_string_hex_to_dec(&mut input_file, &mut output_file),
            SubCommand::DecCon { hex_vec } => {
                api::convert_file_hex_into_dec(&hex_vec);
            }
        },
    }
}
