use std::fs;
use std::str;
use toml;

fn main() {
    let bytes = fs::read("output.toml").expect("must be readable!");
    let s = match str::from_utf8(&bytes) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let  cargo_toml:toml::Value = toml::from_str(s).expect("Invalid TOML detected");
    println!("RAW TOML:");
    println!("{:#?}", cargo_toml);
    let serde_json = cargo_toml.get("dev-dependencies").unwrap().get("serde_json").unwrap();
    println!("JSON Encoded:");
    println!("{}", serde_json.as_str().unwrap());
    let mut copy = cargo_toml.clone();
    let target_node = copy.as_table_mut().unwrap().get_mut("dev-dependencies").unwrap().as_table_mut().unwrap();
    target_node.insert("new-key".to_string(), toml::Value::String("4.888".to_string()));

    fs::write("output_out.toml", copy.to_string()).expect("Could not write to output.toml");
}