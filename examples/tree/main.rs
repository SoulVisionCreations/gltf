
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate gltf;

use gltf::json;
use std::{fs, io};

use std::boxed::Box;
use std::error::Error as StdError;

fn print_tree(node: &gltf::Node, depth: i32) {
    for _ in 0..(depth - 1) {
        print!("  ");
    }
    print!(" -");
    let index = node.index();
    let name = node.name().unwrap_or("<Unnamed>");
    println!(" Node {} ({})", index, name);
    for child in node.children() {
        print_tree(&child, depth + 1);
    }
}

fn run(path: &str) -> Result<(), Box<StdError>> {
    let file = fs::File::open(&path)?;
    let buf_reader = io::BufReader::new(file);
    let json: json::Root = json::from_reader(buf_reader)?;
    let gltf = gltf::Gltf::from_json(json);
    for scene in gltf.scenes() {
        let index = scene.index();
        let name = scene.name().unwrap_or("<Unnamed>");
        println!("Scene {} ({})", index, name);
        for node in scene.nodes() {
            print_tree(&node, 1);
        }
    }
    Ok(())
}

fn main() {
    if let Some(path) = std::env::args().nth(1) {
        let _ = run(&path).expect("runtime error");
    } else {
        println!("usage: gltf-tree <FILE>");
    }
}