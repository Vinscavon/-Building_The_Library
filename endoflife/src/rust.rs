use serde::{
    Serialize, Deserialize
};


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct RustSingleCircle {
 pub release_date: Option<String>,
 pub eol: bool,
 pub lastest: Option<String>,
 pub lastest_release: Option<String>,
 pub lts: bool
}