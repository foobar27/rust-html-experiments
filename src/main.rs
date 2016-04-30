// Copyright 2014 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate html5ever;

mod cache;
mod download;

use cache::Cache;

#[macro_use]
extern crate string_cache;
extern crate tendril;

use std::io::{self, Read};
use std::iter::repeat;
use std::default::Default;

use tendril::{StrTendril,TendrilSink};
use html5ever::parse_document;
use html5ever::rcdom::{Document, Doctype, Text, Comment, Element, RcDom, Handle};


// TODO factor out into a 'Cache' trait/struct?
// static CACHE_DIR: &'static Path = Path::new("tmp/cache/download");

// fn walk_iterative(handle: Handle) {
//     let mut stack = vec![handle];
//     while let Some(x) = stack.pop() {
//         let node = handle.borrow();
//         // match node.node {
//         //     Document
//         //         => 
//         // }
//         // for child in node.children.iter() {
//         //     stack.push(&child);
//         // }
//     }
// }

fn walk(indent: usize, handle: Handle) {
    let node = handle.borrow();
    // FIXME: don't allocate
    print!("{}", repeat(" ").take(indent).collect::<String>());
    match node.node {
        Document
            => println!("#Document"),

        Doctype(ref name, ref public, ref system)
            => println!("<!DOCTYPE {} \"{}\" \"{}\">", *name, *public, *system),

        Text(ref text)
            => println!("#text: {}", escape_default(text)),

        Comment(ref text)
            => println!("<!-- {} -->", escape_default(text)),

        Element(ref name, _, ref attrs) => {
            assert!(name.ns == ns!(html));
            print!("<{}", name.local);
            for attr in attrs.iter() {
                assert!(attr.name.ns == ns!());
                print!(" {}=\"{}\"", attr.name.local, attr.value);
            }
            println!(">");
        }
    }

    for child in node.children.iter() {
        walk(indent+4, child.clone());
    }
}

// FIXME: Copy of str::escape_default from std, which is currently unstable
fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

fn main() {
    //let stdin = io::stdin();
    let cache = cache::DiskCache {
        directory: "tmp/cache/download".to_string(),
        compute: &download::download
    };
    let data = cache.get("http://doc.rust-lang.org/");
    //let mut result = data.bytes();
    //res.read_to_end(&mut result);

    let data = StrTendril::from_slice(&data);
    
    //let chars = data.chars();
    let dom = parse_document(RcDom::default(), Default::default())
        .one(data.clone())
        //.from_bytes(result)
        //.from_utf8()
    //        .read_from(&mut stdin.lock())
        //.read_from(&mut data)
    //.unwrap();
        ;
    walk(0, dom.document);

    if !dom.errors.is_empty() {
        println!("\nParse errors:");
        for err in dom.errors.into_iter() {
            println!("    {}", err);
        }
    }
}
