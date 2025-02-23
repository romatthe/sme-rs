use std::fs::File;
use std::io::Read;

use crate::appinfo::{AppInfoParserPacker, AppPatch};
use nom::Finish;
use sha1::{Digest, Sha1};
use crate::vdf::serializer::VdfSerializer;

mod v29;
mod vdf;
mod appinfo;

fn main() -> anyhow::Result<()> {
    let file_read = File::open("appinfo.vdf.pristine")?;
    let mut app_info = v29::AppInfo::parse(file_read)?;
    // let mut file_create = File::create("appinfo_duplicated.vdf")?;

    let app = app_info.apps.get(&1325200).unwrap();
    let serializer = VdfSerializer::new(&app_info.table);
    let serialized = serializer.serialize_vdf(&app.vdf)?;

    println!("{}", serialized);

    let mut file_create = File::create("appinfo.vdf.duplicated")?;
    app_info.pack(file_create)?;


    // v29::packer::pack_app_info(&mut file_create, &app_info)?;
    // drop(file_create);

    // let file_read2 = File::open("appinfo_duplicated.vdf")?;
    // let app_info2 = v29::AppInfo::parse(file_read2)?;

    // Test patching
    // let patch = AppPatch { appid: 1325200, name: "Biden".to_string(), sort_as: None, };
    // app_info.patch_app(patch)?;
    // app_info.pack(file_create)?;

    // let file_read = File::open("appinfo_duplicated.vdf")?;
    // let app_info2 = v29::AppInfo::parse(file_read)?;
    // println!("{:?}", app_info2.apps.get(&1325200));

    // let app = app_info.apps.get(&1325200).unwrap();
    // let mut vdf_buffer = Vec::new();
    // vdf::packer::pack_vdf(&mut vdf_buffer, app.vdf.as_slice())?;
    // let (_, parsed) = vdf::parser::parse_vdf_nodes(vdf_buffer.as_slice())?;


    // app_info.pack(file_create)?;

    // for app in app_info.apps {
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
    // }

    // app_info.pack(file_write)?;

    // println!("Apps: {}", app_info.apps.len());

    // let app = app_info.apps.iter().find(|app| app.appid == 1325200).unwrap();
    // let app = app_info.apps.iter().find(|app| app.appid == 1072420).unwrap();
    // let app = app_info.apps.get(&1325200).unwrap();
    // let app = app_info.apps.iter().find(|app| app.appid == 7).unwrap();

    // let serializer = VdfSerializer::new(&app_info.table);
    // let serialized = serializer.serialize_vdf(&app.vdf)?;
    // println!("{}", serialized);

    // let mut buffer_good = String::new();
    // let mut buffer_bad = String::new();
    // let mut counter_good = 0;
    // let mut counter_bad = 0;
    //
    // for (appid, app) in app_info.apps {
    //     let serialized = serializer.serialize_vdf(&app.vdf)?;
    //
    //     let mut hasher = Sha1::new();
    //     hasher.update(serialized.as_bytes());
    //
    //     let sha1_original = &app.sha1_text;
    //     let sha1_calculated = hasher.finalize();
    //
    //     if *sha1_original == *sha1_calculated {
    //         // println!("Original  : {:?}", sha1_original.as_slice());
    //         // println!("Calculated: {:?}", sha1_calculated.as_slice());
    //         // println!("{serialized}");
    //
    //         buffer_good.push_str(&serialized);
    //         counter_good += 1;
    //     } else {
    //         buffer_bad.push_str(&serialized);
    //         counter_bad += 1;
    //     }
    // }

    // println!("{}", buffer_bad);
    // println!("Count good: {}", counter_good);
    // println!("Count bad:  {}", counter_bad);

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