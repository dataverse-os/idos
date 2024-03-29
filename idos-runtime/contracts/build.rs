use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("addresses.rs");
    let mut f = File::create(&dest_path).unwrap();

    let data = std::fs::read_to_string("addresses.json").unwrap();
    let v: Value = serde_json::from_str(&data).unwrap();

    let polygon_mumbai = &v["PolygonMumbai"];

    write!(
        f,
        "
        pub static DATAVERSE_RELAYER: &'static str = \"{}\";
        pub static DATAVERSE_VERIFIER: &'static str = \"{}\";
        ",
        polygon_mumbai["DataverseRelayer"].as_str().unwrap(),
        polygon_mumbai["DataverseVerifier"].as_str().unwrap()
    )
    .unwrap();
}
