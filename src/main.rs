extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let app = App::new("tp")
                       .version("0.1.0")
                       .author("Calvin D. Woo <calvin.d.woo@gmail.com>")
                       .about("Creates warp points for teleporting around the filesystem.")
                       .subcommand(SubCommand::with_name("add")
                                   .about("Add a warp point that allows us to come back to here")
                                   .arg(Arg::with_name("warpname")
                                        .required(true)
                                        .help("Name of warp point"))
                                   .arg(Arg::with_name("warppath")
                                        .help("Path to set warp point. Defaults to current path")))
                       .subcommand(SubCommand::with_name("list")
                                   .about("List all warp points"))
                       .subcommand(SubCommand::with_name("warp")
                                   .about("Warp to a specified point")
                                   .arg(Arg::with_name("warp point")
                                        .required(true)
                                        .help("Warp point to warp to")))
                       .subcommand(SubCommand::with_name("remove")
                                   .about("Remove an existing warp point")
                                   .arg(Arg::with_name("warp point")
                                        .required(true)
                                        .help("Warp point to warp to")));

    let matches = app.get_matches();
}
