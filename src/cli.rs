use clap::{ArgAction, Parser};

#[derive(Parser)]
#[command(name = "ATTT")]
#[command(version = "1.0")]
#[command(about = "Run the ATTT appliction", long_about = None)]
pub struct Cli {
    #[arg(long,action=ArgAction::Set,default_value_t=false)]
    pub skip_menu: bool,
    #[arg(long,action=ArgAction::Set,default_value_t=String::new())]
    pub config: String,
    #[arg(long,action=ArgAction::Set,default_value_t=25)]
    pub wc: u32,
}
