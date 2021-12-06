extern crate clap;
extern crate ansi_term;

use std::env::var;
use std::path::{Path, PathBuf};
use std::fs::{OpenOptions, canonicalize};
use std::io::{Read, Write};
use std::process::exit;

use serde::{Serialize, Deserialize};
use clap::{Arg, App, SubCommand};

#[derive(Serialize, Deserialize, Debug)]
struct WarpPoint {
    id: i32,
    name: String,
    path: PathBuf,
}

impl WarpPoint {
    fn new(id: i32, name: String, path: PathBuf) -> Self {
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

    match matches.subcommand() {
        ("add", Some(add_matches)) => {
            let warppath = match add_matches.value_of("warppath") {
                Some(wp_path) => wp_path,
                None => ".",
            };
            add_warp_point(&mut metadata_vec, add_matches.value_of("warpname").unwrap(), warppath);
        },
        ("list", _) => list_warp_points(&metadata_vec),
        ("remove", Some(remove_matches)) => {
            let warpname = match remove_matches.value_of("warp point") {
                Some(wp_name) => wp_name,
                None => ".",
            };
            remove_warp_point(&mut metadata_vec, warpname);
        },
        ("warp", Some(warp_matches)) => {
            let warpname = match warp_matches.value_of("warp point") {
                Some(wp_name) => wp_name,
                None => "."
            };
            activate_warp_point(&mut metadata_vec, warpname);
        },
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
    println!("teleport points: {}", 
             ansi_term::Colour::Blue
                .bold()
                .paint(format!("(total {})", metadata_vec.len())));

    // iterate through metadata and list warp points
    for wp in metadata_vec {
        println!("{}\t{}",
                 wp.name,
                 ansi_term::Colour::Blue
                    .bold()
                    .paint(format!("{}", wp.path.to_string_lossy())));
    }
}

fn add_warp_point(metadata_vec: &mut Vec<WarpPoint>, warpname: &str, warppath: &str) {
    // check if warp point already exists
    for wp in metadata_vec.iter() {
        if warpname == wp.name {
            println!("warp point already created with that name!");
            return
        }
    }

    println!("creating warp point:");
    let n_warp_points: i32 = metadata_vec.len() as i32;
    let canonicalized_warp_path = canonicalize(&PathBuf::from(warppath)).unwrap();

    println!("{}\t{}",
             warpname,
             ansi_term::Colour::Blue
                    .bold()
                    .paint(format!("{}", &canonicalized_warp_path.to_string_lossy())));

    let new_warppoint: WarpPoint = WarpPoint::new(n_warp_points, warpname.to_string(), canonicalized_warp_path);
    metadata_vec.push(new_warppoint);
}

fn remove_warp_point(metadata_vec: &mut Vec<WarpPoint>, warpname: &str) {
    // iterate through metadata to remove named warppoint and shift ids
    let mut id: i32 = 0;
    let pre_remove_len: i32 = metadata_vec.len() as i32;

    metadata_vec
        .iter_mut()
        .for_each(|item: &mut WarpPoint| {
            if item.name == warpname {
                item.id = -1;
                println!("removed warp point [{}]", 
                         ansi_term::Colour::Blue
                                .bold()
                                .paint(format!("{}", warpname)));
            } else {
                item.id = id;
                id += 1;
            }
        });

    metadata_vec.retain(|item: &WarpPoint| item.id > -1);
    if pre_remove_len == (metadata_vec.len() as i32) {
        println!("no warp point removed");
    }
}

fn activate_warp_point(metadata_vec: &Vec<WarpPoint>, warpname: &str) {
    // iterate through warp points, emitting path name at match
    for wp in metadata_vec {
        if wp.name == warpname {
            println!("{}", wp.path.to_string_lossy());
            exit(4);
        }
    }

    // output error message if no warppoint found with that name
    println!("no warp point found with that name!");
}