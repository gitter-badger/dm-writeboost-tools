extern crate clap;
extern crate getopts;
extern crate lib;

use std::process::Command;
use std::env;
use clap::{Arg, App};

fn main() {
    let matches = App::new("wbremove")
        .version("0.1")
        .author("Akira Hayakawa <ruby.wkkt@gmail.com>")
        .about("Remove a writeboost device")
        .arg(Arg::with_name("LVNAME")
             .required(true)
             .index(1))
        .arg(Arg::with_name("noflush")
             .help("Don't flush RAM buffer to caching device before removing")
             .long("noflush"))
        .arg(Arg::with_name("nowriteback")
             .help("Don't write back dirty caches to the backing device before removing")
             .long("nowriteback"))
        .get_matches();

    let wbname = matches.value_of("LVNAME").unwrap().to_string();

    if !matches.is_present("noflush") {
        Command::new("dmsetup")
            .arg("suspend")
            .arg(&wbname)
            .spawn()
            .expect("Failed to flush transient data");

        Command::new("dmsetup")
            .arg("resume")
            .arg(&wbname)
            .spawn()
            .expect("Failed to flush transient data");
    }

    if !matches.is_present("nowriteback") {
        Command::new("dmsetup")
            .arg("message")
            .arg(&wbname)
            .arg("0")
            .arg("drop_caches")
            .spawn()
            .expect("Failed to drop caches");

        let wbdev = lib::WBDev::new(wbname.to_string());
        Command::new("dd")
            .arg("if=/dev/zero")
            .arg(format!("of={}", wbdev.caching_dev_name()))
            .arg("bs=512")
            .arg("count=1")
            .spawn()
            .expect("Failed to zero out the caching device");
    }

    Command::new("dmsetup")
        .arg("remove")
        .arg(&wbname)
        .spawn()
        .expect("Failed to execute dmsetup create");
}
