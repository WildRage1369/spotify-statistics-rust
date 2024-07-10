use convert_case::{Case, Casing};
use fancy::printcoln;
use regex::Regex;
use serde_json::Value;
use std::env;
use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let stat_opts: [&str; 3] = ["song", "album", "artist"];

    let dir_arg: String;
    let stat_arg: String;
    let title_arg: String;
    match Vec::len(&args) {
        1 => {
            help();
            return Ok(());
        }
        2 => {
            if args[1] == "--help" {
                help();
            } else {
                println!("ERROR: not enough arguments passed");
            }
            return Ok(());
        }
        3 => {
            println!("ERROR: not enough arguments passed");
            return Ok(());
        }
        4 => {
            dir_arg = args[1].to_lowercase();
            stat_arg = args[2].to_lowercase();
            title_arg = args[3].to_lowercase();

            if !stat_opts.contains(&stat_arg.as_str()) {
                println!("ERROR: stat argument is invalid, valid options are 'song', 'album', and 'artist'");
                return Ok(());
            }
        }
        _ => {
            println!("ERROR: too many arguments passed");
            return Ok(());
        }
    }

    let mut count = 0;
    for path in fs::read_dir(dir_arg)? {
        let item = path?.path();
        let item_path: &str = item.to_str().unwrap();

        let re = Regex::new(r".*\.json").unwrap();
        if re.find(item_path) == None {
            continue;
        };
        let file_as_str = match fs::read_to_string(item_path) {
            Ok(suc) => suc,
            Err(_) => continue,
        };
        let json: Value = serde_json::from_str(&file_as_str)?;
        for item in json.as_array().unwrap() {
            let item_stat: &str;
            match stat_arg.as_str() {
                "song" => {
                    item_stat = item["master_metadata_track_name"].as_str().unwrap_or("");
                }
                "album" => {
                    item_stat = item["master_metadata_album_album_name"]
                        .as_str()
                        .unwrap_or("");
                }
                "artist" => {
                    item_stat = item["master_metadata_album_artist_name"]
                        .as_str()
                        .unwrap_or("");
                }
                _ => item_stat = "",
            }
            if item_stat.to_lowercase() == title_arg {
                count += 1;
            }
        }
    }
    println!(
        "You have listened to {stat_arg} {} {count} times",
        title_arg.to_case(Case::Title)
    );
    Ok(())
}

fn help() {
    printcoln!("\
[b|u]spotify-parser[:]
    A program to parse Spotify's Extended Streaming History Data written in Rust

[b|u]Usage[:]
    [b]spotify-parser[:] <dir> <stat> <name>

[b|u]Options:[:]
    <dir>           Directory of the Extended Streaming History, can be relative
    <stat>          One of: 'song', 'album' or 'artist'
    <name>          The name of said song/album/artist");
}
