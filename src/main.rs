extern crate clap;

use std::env::var;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Read;

use serde::{Serialize, Deserialize};
use clap::{Arg, App, SubCommand};

#[derive(Serialize, Deserialize, Debug)]
struct WarpPoint {
    id: i32,
    name: String,
    path: String,
}

impl WarpPoint {
    fn new(id: i32, name: String, path: String) -> Self {
        WarpPoint { id, name, path }
    }
}

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

    // initialize tp metadata
    let metadata_path: String = format!("{}/.tp/points.json", var("HOME").unwrap());
    if !Path::new(&metadata_path).exists() {
        println!("First time run: `tp` metadata initialized.");
    }

    println!("{}", metadata_path);
    let metadata_file = OpenOptions::new()
                                .write(true)
                                .create(true)
                                .open("~/points.json")
                                .unwrap();

    // let mut metadata_file = File::open(&metadata_path).unwrap();
    println!("lol");
    // let mut metadata: String = String::new();
    // metadata_file.read_to_string(&mut metadata).unwrap();

    // let mut metadata_vec: Vec<WarpPoint> = serde_json::from_str(&metadata).unwrap();
    // println!("{:?}", metadata_vec);
}
