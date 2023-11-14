use qrcode_generator::QrCodeEcc;
use web_view::*;

fn main() {
    // word to encode
    let word = "example.com";

    let result: String =
        qrcode_generator::to_svg_to_string(word, QrCodeEcc::Low, 256, None::<&str>).unwrap();

    web_view::builder()
        .title("QR Code Viewer")
        .content(Content::Html(result))
        .size(256, 256)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
