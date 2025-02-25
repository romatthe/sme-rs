use std::fmt;
use std::fs::File;
use std::str::FromStr;
use anyhow::Result;
use serde::{Deserialize, Deserializer};
use serde::de::{MapAccess, Visitor};

pub trait AppInfoParserPacker {
    fn parse(app_info_file: File) -> Result<Self> where Self: Sized;
    fn pack(self, app_info_file: File) -> Result<()>;
    fn patch_app(&mut self, patch: AppPatch) -> Result<()>;
}

#[derive(Debug, Deserialize)]
pub struct AppPatch {
    pub appid: u32,
    pub name: String,
    pub sort_as: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(deserialize_with = "de_patchmap")]
    pub patches: Vec<AppPatch>,
}


fn de_patchmap<'de, D>(deserializer: D) -> Result<Vec<AppPatch>, D::Error>
where
    D: Deserializer<'de>,
{
    struct PatchMap;

    impl<'de> Visitor<'de> for PatchMap {
        type Value = Vec<AppPatch>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map of name to AppPatch")
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some((appid, mut patch)) = map.next_entry::<String, AppPatch>()? {
                patch.appid = u32::from_str(&appid).unwrap();
                vec.push(patch);
            }
            Ok(vec)
        }
    }

    deserializer.deserialize_map(PatchMap)
}