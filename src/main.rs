use clap::Parser;
use std::process::Command;
use std::{
    fs::File,
    io::Write,
};
use std::io::{self, Read};

mod utils;

#[derive(Parser)]
struct Args {
    /// file path
    #[arg(short)]
    file_path: std::path::PathBuf,

    // should open in browser
    #[arg(short, default_value_t = true)]
    open_browser: bool,
}


fn open_in_browser(path: &std::path::Path) {
    Command::new("open")
        .arg(path)
        .spawn()
        .expect("failed to open link");
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut buffer = "".to_string();
    io::stdin().read_to_string(&mut buffer)?;

    let markers = utils::read_stdin(buffer)?;
    let serialized = serde_json::to_string(&markers).unwrap();
    let markers_template = format!("var markers = {}", serialized);

    let template_path = std::path::Path::new("./template/index.html");
    let template = std::fs::read_to_string("./template/index.html")?;
    let page = template.replace("var markers = []", &markers_template);

    let mut file = File::create(args.file_path)?;
    file.write_all(page.as_bytes())?;

    if args.open_browser {
        open_in_browser(template_path);
    }

    return Ok(());
}
