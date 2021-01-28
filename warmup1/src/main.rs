use serde::{Serialize, Deserialize};
use ring::{digest};

#[derive(Serialize, Deserialize, Debug)]
struct NameHash {
    name: String,
    hash: String
}

fn main() {
    let my_name = String::from("Yichi Zhang");
    let name_bytes: &[u8] = my_name.as_bytes();
    // println!("{:?}", name_bytes);
    let my_hash = digest::digest(&digest::SHA256, name_bytes);
    let hex_hash = hex::encode(my_hash.as_ref());
    // println!("{:?}", hex_hash);
    let name_hash = NameHash {name: my_name, hash: hex_hash};
    let encoded: Vec<u8> = bincode::serialize(&name_hash).unwrap();
    let decoded: NameHash = bincode::deserialize(&encoded[..]).unwrap();
    println!("{:?}", encoded);
    println!("{:?}", decoded);
}
