// src/utils/commands.rs
// author: steinkirch

use clap::{Args, Parser, Subcommand};


#[derive(Debug, Parser)]
#[clap(name = "w3kit")]
#[clap(about = "🕹✨ a rusty toolkit for several blockchains ✨🕹")]
pub struct CliStruct {
    #[clap(subcommand)]
    pub command: CliEnum,
}


#[derive(Debug, Subcommand)]
pub enum CliEnum {
    #[clap(arg_required_else_help = true)]
    /// Test a websocket connection to a given blockchain
    WS(ConnectionArgs),
    #[clap(arg_required_else_help = true)]
    /// Test an http connection to a given blockchain
    HTTP(ConnectionArgs),
}


#[derive(Debug, Args)]
pub struct ConnectionArgs {
    #[clap(short, long)]
    /// The blockchain to connect to
    pub blockchain: String,
    #[clap(short, long)]
    /// The account to be fetched 
    pub account: String,
}
