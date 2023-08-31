extern crate clap;
extern crate image;
extern crate qrcode;
extern crate thiserror;

use std::io;
use std::path::{Path, PathBuf};
use std::process;

use clap::Parser;
use image::Luma;
use qrcode::types::QrError;
use qrcode::QrCode;

#[derive(Debug, Parser)]
#[command(author, version, about = "QRコードを生成します。")]
struct GenerateOptions {
    /// QRコードに埋め込む文字列を指定してください。
    text: String,

    /// 出力先のファイルパスを指定してください。省略した場合は標準出力にQRコードを表示します。
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
}

enum Command {
    GenerateCode(GenerateOptions),
}

type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("qr code error")]
    QrCode(#[from] QrError),

    #[error("io error")]
    Io(#[from] io::Error),

    #[error("image error")]
    Image(#[from] image::ImageError),
}

fn parse_command() -> AppResult<Command> {
    let generate_options = GenerateOptions::parse();
    Ok(Command::GenerateCode(generate_options))
}

fn output_file(code: &QrCode, path: &Path) -> AppResult<()> {
    // 画像に変換する
    let image = code.render::<Luma<u8>>().build();

    // ファイル出力する
    image.save(path)?;

    Ok(())
}

fn output_stdout(code: &QrCode) -> AppResult<()> {
    // 文字列に変換する
    let text = code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();

    // 標準出力に表示する
    println!("{}", text);

    Ok(())
}

fn generate_code(generate_options: &GenerateOptions) -> AppResult<()> {
    // QRコード生成
    let code = QrCode::new(generate_options.text.as_bytes())?;

    // 出力先が指定されている場合は画像に、ない場合は標準出力にテキストで表示する
    match generate_options.output.as_ref() {
        Some(ref path) => output_file(&code, path),
        None => output_stdout(&code),
    }
}

fn run() -> AppResult<()> {
    match parse_command()? {
        Command::GenerateCode(generate_options) => generate_code(&generate_options),
    }
}

fn main() {
    process::exit(match run() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("error: {}", e);
            -1
        }
    })
}
