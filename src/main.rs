use std::fs::File;
use std::io;
use std::io::Read;

use nom::Finish;
use sha1::digest::DynDigest;
use crate::appinfo::AppInfoParserPacker;
use crate::vdf::parser::{VdfNode, VdfString};

mod v29;
mod vdf;
mod appinfo;

fn main() -> anyhow::Result<()> {
    let file = File::open("appinfo.vdf")?;
    let app_info = v29::AppInfo::parse(file)?;

    // println!("Apps: {}", app_info.apps.len());

    // let app = app_info.apps.iter().find(|app| app.appid == 1325200).unwrap();
    let app = app_info.apps.iter().find(|app| app.appid == 1072420).unwrap();

    // println!("Found: {:?}", app.vdf);
    println!("StringRef name: {:?}", app_info.table[4]);
    // println!("StringRef name_localized: {:?}", app_info.table[474]);

    println!(
        "{0: <40} | {1: <20} | {2: <80} | {3: <20} | {4: <40}",
        "key value", "key ref", "value value", "value ref", "value type"
    );

    // display_the_node(&app.vdf, &app_info.table);

    println!("Top-level: ");

    let mut vec = Vec::new();

    if let VdfNode::Nested { key, nodes } = &app.vdf[0][3] {
        vec = nodes.to_vec();
    }

    for n in vec {
        match n {
            VdfNode::Nested { key, .. } => {
                let ref_key = decode_the_string(&key, &app_info.table);
                println!("Entry Name: {} - Entry Ref: {}", ref_key.0, ref_key.1);
            },
            VdfNode::String { key, .. } => {
                let ref_key = decode_the_string(&key, &app_info.table);
                println!("Entry Name: {} - Entry Ref: {}", ref_key.0, ref_key.1);
            },
            VdfNode::Int { key, .. } => {
                let ref_key = decode_the_string(&key, &app_info.table);
                println!("Entry Name: {} - Entry Ref: {}", ref_key.0, ref_key.1);
            }
        }
    }

    // for node in app.vdf {
    //     println!(
    //         "{0: <10} | {1: <10} | {2: <10}",
    //         "Key", "Value", "RefValue"
    //     );
    //
    //
    //     // match node {
    //     //     VdfNode::Nested { .. } => {}
    //     //     VdfNode::String { key, value } => {}
    //     //     VdfNode::Int { key, value } => {
    //     //         println!(
    //     //             "{0: <10} | {1: <10} | {2: <10}",
    //     //             key, value, ""
    //     //         );
    //     //     }
    //     // }
    //
    // }
    //


    Ok(())
}

fn display_the_node(nodes: &Vec<VdfNode>, table: &Vec<String>) {
    for node in nodes {
        match node {
            VdfNode::Nested { key, nodes } => display_the_node(nodes, table),
            VdfNode::String { key, value } => {
                let (ref_key, ref_str) = (decode_the_string(key, table), decode_the_string(value, table));
                let t = match value {
                    VdfString::StringRef(_) => "StringRef",
                    VdfString::String(_) => "String"
                };
                println!(
                        "{0: <40} | {1: <20} | {2: <80} | {3: <20} | {4: <40}",
                        ref_key.0, ref_key.1, ref_str.0, ref_str.1, t
                );
            },
            VdfNode::Int { key, value } => {
                let ref_key = decode_the_string(key, table);
                println!(
                    "{0: <40} | {1: <20} | {2: <80} | {3: <20} | {4: <40}",
                    ref_key.0, ref_key.1, value, "", "Int"
                );
            }
        }
    }
}

fn decode_the_string(string: &VdfString, table: &Vec<String>) -> (String, String) {
    match string {
        VdfString::StringRef(n) => {
            (table[*n as usize].to_string(), n.to_string())
        }
        VdfString::String(s) => {
            (s.to_string(), "".to_string())
        }
    }
}

fn display_the_string(string: &VdfString, table: &Vec<String>) -> String {
    match string {
        VdfString::StringRef(n) => {
            table[*n as usize].to_string()
        }
        VdfString::String(s) => {
            s.to_string()
        }
    }
}
