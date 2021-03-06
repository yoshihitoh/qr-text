extern crate clap;
extern crate image;
extern crate qrcode;

#[macro_use]
extern crate failure;

use std::io;
use std::path::{Path, PathBuf};
use std::process;

use clap::{App, Arg};

use image::Luma;

use qrcode::QrCode;
use qrcode::types::QrError;

struct GenerateOptions {
    text: String,
    output: Option<PathBuf>,
}

enum Command {
    GenerateCode(GenerateOptions),
}

type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Fail)]
enum AppError {
    #[fail(display = "qr code error: {}", err)]
    QrError { err: QrError },

    #[fail(display = "io error: {}", err)]
    IoError { err: io::Error },
}

impl From<QrError> for AppError {
    fn from(err: QrError) -> Self {
        AppError::QrError { err }
    }
}

impl From<io::Error> for AppError {
    fn from(err: io::Error) -> Self {
        AppError::IoError { err }
    }
}

fn parse_command() -> AppResult<Command> {
    let matches = App::new("qr-text")
        .version("0.0.1")
        .author("yoshihitoh")
        .about("指定した文字列のQRコードを生成します。")
        .arg(
            Arg::with_name("OUTPUT")
                .short("o")
                .long("output")
                .value_name("FILE")
                .takes_value(true)
                .help("出力先のファイルパスを指定してください。"),
        )
        .arg(
            Arg::with_name("TEXT")
                .required(true)
                .help("QRコードに埋め込む文字列を指定してください。"),
        )
        .get_matches();

    // text: NOTE: required指定のパラメタなので unwrap で取り出す
    let text = String::from(matches.value_of("TEXT").unwrap());
    let output = matches.value_of("OUTPUT").map(PathBuf::from);

    let generate_options = GenerateOptions { text, output };
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
    let text = code.render::<char>()
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
