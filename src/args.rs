use clap::Parser;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Display, Debug, Clone, Copy, EnumString, Serialize, Deserialize)]
pub enum Case {
    #[strum(serialize = "cC")]
    Camel,
    #[strum(serialize = "c_c")]
    Snake,
    #[strum(serialize = "cc")]
    Lower,
    #[strum(serialize = "CC")]
    Upper,
    #[strum(serialize = "c-c")]
    Kebab,
    #[strum(serialize = "C-C")]
    UpperKebab,
}
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
///To Case is a command line case-changing tool especially designed for the Helix Editor
pub struct Args {
    /// Case name (cC, c_c, CC, cc) (if not defined, the last choice will be used)
    #[arg()]
    pub case: Option<Case>,
    /// Word to change (taken from stdin if not defined)
    #[arg(short, long)]
    pub word: Option<String>,
}
