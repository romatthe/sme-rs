use std::fs::File;
use std::io::Read;

use crate::appinfo::AppInfoParserPacker;
use crate::vdf::parser::{VdfNode, VdfString};
use nom::Finish;
use sha1::{Digest, Sha1};

mod v29;
mod vdf;
mod appinfo;

fn main() -> anyhow::Result<()> {
    let file_read = File::open("appinfo.vdf")?;
    let mut file_create = File::create("appinfo_duplicated.vdf")?;

    let app_info = v29::AppInfo::parse(file_read)?;
    v29::packer::pack_app_info(&mut file_create, &app_info)?;
    drop(file_create);

    let file_read2 = File::open("appinfo_duplicated.vdf")?;
    let app_info2 = v29::AppInfo::parse(file_read2)?;

    for app in app_info.apps {
        // let mut buffer = Vec::new();
        // vdf::packer::pack_vdf(&mut buffer, app.vdf.as_slice())?;
        // let (_, app2) = vdf::parser::parse_vdf_nodes(buffer.as_slice())?;
        // v29::packer::pack_app_info()
        //
        // assert_eq!(app.appid, );
        // assert_eq!(app.blob, buffer);
        // assert_eq!(app.blob, buffer);
        //
        //
        //
        // if app.blob != buffer {
        //     println!("SPAGHETTI");
        // }
    }

    // app_info.pack(file_write)?;

    // println!("Apps: {}", app_info.apps.len());

    // let app = app_info.apps.iter().find(|app| app.appid == 1325200).unwrap();
    // let app = app_info.apps.iter().find(|app| app.appid == 1072420).unwrap();

    // println!("Found: {:?}", app.vdf);
    // println!("StringRef name: {:?}", app_info.table[4]);
    // println!("StringRef name_localized: {:?}", app_info.table[474]);

    // println!(
    //     "{0: <40} | {1: <20} | {2: <80} | {3: <20} | {4: <40}",
    //     "key value", "key ref", "value value", "value ref", "value type"
    // );

    // display_the_node(&app.vdf, &app_info.table);

    // println!("Top-level: ");
    //
    // let mut vec = Vec::new();
    //
    // if let VdfNode::Nested { key, nodes } = &app.vdf[0] {
    //     if let VdfNode::Nested { key, nodes } = &nodes[1] {
    //         // if let VdfNode::Nested { key, nodes } = &nodes[5] {
    //             vec = nodes.to_vec();
    //         // }
    //     }
    // }

    // for n in vec {
    //     match n {
    //         VdfNode::Nested { key, .. } => {
    //             let ref_key = decode_the_string(&key, &app_info.table);
    //             println!("Entry Name: {} - Entry Ref: {}", ref_key.0, ref_key.1);
    //         },
    //         VdfNode::String { key, .. } => {
    //             let ref_key = decode_the_string(&key, &app_info.table);
    //             println!("Entry Name: {} - Entry Ref: {}", ref_key.0, ref_key.1);
    //         },
    //         VdfNode::Int { key, .. } => {
    //             let ref_key = decode_the_string(&key, &app_info.table);
    //             println!("Entry Name: {} - Entry Ref: {}", ref_key.0, ref_key.1);
    //         }
    //     }
    // }

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

    // for app in app_info.apps {
    //     let mut buffer = Vec::new();
    //     let blob1 = app.blob.as_slice();
    //     let blob2 = vdf::packer::pack_vdf(&mut buffer, &app.vdf)?;
    //     let blob2 = buffer.as_slice();
    //
    //     assert_eq!(blob1, blob2);
    //     println!("Another one");
    //     if blob1 != blob2 {
    //         panic!("Uh oh");
    //     }
    // }

    // let mut hasher = Sha1::new();
    // hasher.update(&app.blob);
    //
    // let sha1_original = &app.sha1_binary;
    // let sha1_calculated = hasher.finalize();

    // println!("Original  : {:?}", sha1_original.as_slice());
    // println!("Calculated: {:?}", sha1_calculated.as_slice());

    // Let's test the packer
    // v29::packer::pack_app_info(&mut file, &app_info)?;


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
                // println!(
                //         "{0: <40} | {1: <20} | {2: <80} | {3: <20} | {4: <40}",
                //         ref_key.0, ref_key.1, ref_str.0, ref_str.1, t
                // );
            },
            VdfNode::Int { key, value } => {
                let ref_key = decode_the_string(key, table);
                // println!(
                //     "{0: <40} | {1: <20} | {2: <80} | {3: <20} | {4: <40}",
                //     ref_key.0, ref_key.1, value, "", "Int"
                // );
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
            (s.clone().into_string().unwrap(), "".to_string())
        }
    }
}

fn display_the_string(string: &VdfString, table: &Vec<String>) -> String {
    match string {
        VdfString::StringRef(n) => {
            table[*n as usize].to_string()
        }
        VdfString::String(s) => {
            s.clone().into_string().unwrap()
        }
    }
}
