# spotify-statistics-rust

A CLI program to parse Spotify's Extended Streaming History Data written in Rust

# Installation

Download the latest release to your directory of choice

# Usage

*spotify-parser* \<dir\> \<stat\> \<name\>

 | Argument | Description |
 |------------|-----------|
 | \<dir\> | Directory of the Extended Streaming History, can be relative |
 | \<stat\> | One of: 'song', 'album' or 'artist' |
 | \<name\> | The name of said song/album/artist |

# Crates.io Dependencies
- convert_case: "0.6.0"
- fancy: "0.3.1"
- json-parser: "1.0.2"
- regex: "1.10.5"
- serde_json: "1.0.120"
