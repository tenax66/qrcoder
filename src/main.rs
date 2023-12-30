use clap::Parser;
use qrcode_generator::QrCodeEcc;
use web_view::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "The word you want to encode")]
    word: String,

    #[arg(short, long, help = "Output filename (.png)")]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();

    match &args.output {
        None => {
            let result: String =
                qrcode_generator::to_svg_to_string(args.word, QrCodeEcc::Low, 256, None::<&str>)
                    .unwrap();

            // centering svg image
            let html = format!(
                r#"
            <style>
                svg {{
                    display: block;
                    margin: auto;
                }}
            </style>
            {}
            "#,
                result
            );

            web_view::builder()
                .title("QR Code Viewer")
                .content(Content::Html(html))
                .size(300, 280)
                .resizable(true)
                .debug(true)
                .user_data(())
                .invoke_handler(|_webview, _arg| Ok(()))
                .run()
                .unwrap();
        }
        Some(output) => {
            qrcode_generator::to_png_to_file(args.word, QrCodeEcc::Low, 256, output).unwrap();
        }
    }
}
