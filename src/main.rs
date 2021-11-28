extern crate clap;

use std::env::var;
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

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
    let metadata_path: String = format!("{}/.tp/", var("HOME").unwrap());
    if !Path::new(&metadata_path).exists() {
        println!("First time run: `tp` metadata initialized at {}", metadata_path);
        std::fs::create_dir(&metadata_path);
    }

    let mut metadata_file_read = OpenOptions::new()
                                    .read(true)
                                    .write(true)
                                    .create(true)
                                    .open(Path::new(&metadata_path).join("points.json"))
                                    .unwrap();

    // let mut metadata_file = File::open("/home/hyperion/.tp/points.json").unwrap();
    let mut metadata = String::new();
    metadata_file_read.read_to_string(&mut metadata).expect("Unable to read string");

    let mut metadata_vec: Vec<WarpPoint> = match serde_json::from_str(&metadata) {
        Ok(v) => v,
        Err(_) => Vec::<WarpPoint>::new(),
    };

    match matches.subcommand_name() {
        Some("list") => list_warp_points(&metadata_vec),
        _ => println!("No subcommand was used"),
    };

    // write metadata
    let metadata_str: String = serde_json::to_string(&metadata_vec).unwrap();
    let mut metadata_file_write = OpenOptions::new()
                                    .write(true)
                                    .truncate(true)
                                    .open(Path::new(&metadata_path).join("points.json"))
                                    .unwrap();
    writeln!(&mut metadata_file_write, "{}", &metadata_str).unwrap();
}

fn list_warp_points(metadata_vec: &Vec<WarpPoint>) {
    println!("teleport points: (total {})", metadata_vec.len());
}