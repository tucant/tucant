// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{
    fs::File,
    io::{self, stdout, BufRead, Write},
};

use base64::{
    alphabet::{self, Alphabet},
    engine::{self, general_purpose},
    Engine,
};

fn main() -> anyhow::Result<()> {
    let file = File::open("base64.txt").unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    let lines = io::BufReader::new(file).lines();

    let custom: Alphabet = base64::alphabet::Alphabet::new(
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789~-",
    )
    .unwrap();

    let engine: engine::GeneralPurpose =
        engine::GeneralPurpose::new(&custom, general_purpose::NO_PAD);

    for line in lines {
        let result = engine.decode(line?.trim_end_matches("_"))?;
        stdout().write(&result)?;
    }

    Ok(())
}
