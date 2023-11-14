use qrcode_generator::QrCodeEcc;
use web_view::*;

fn main() {
    // word to encode
    let word = "example.com";

    let result: String =
        qrcode_generator::to_svg_to_string(word, QrCodeEcc::Low, 256, None::<&str>).unwrap();

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
