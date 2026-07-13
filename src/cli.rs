use camino::Utf8PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "reimg",
    version,
    about = "*",
    long_about = None
)]
pub struct Cli {
    #[arg(value_name = "PATH")]
    pub path: Utf8PathBuf,
}
