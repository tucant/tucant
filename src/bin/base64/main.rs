use std::{fs::OpenOptions, io::Write};

pub fn main() {
    let output1 = base64::decode_config("", base64::URL_SAFE).unwrap();

    println!("{}", hex::encode(&output1));

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("test.bin")
        .unwrap();
    file.write_all(&output1).unwrap();
}
