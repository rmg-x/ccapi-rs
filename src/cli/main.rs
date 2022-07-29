use anyhow::Result;
use ccapi::CCAPI;
use getopts::Options;
use std::env;
use std::net::Ipv4Addr;

mod command;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.reqopt("i", "ip-address", "Console IPv4 address", "");
    opts.reqopt("c", "command", "Command", "");

    let matches = opts.parse(&args[1..])?;

    let ip: Ipv4Addr = matches.opt_str("ip-address").unwrap().parse()?;

    let ccapi = CCAPI::new(ip);

    command::run(&ccapi, &matches)?;

    Ok(())
}
