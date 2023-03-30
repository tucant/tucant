// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::{
    fs::File,
    io::{self, stdout, BufRead, Write},
};

use base64::{
    alphabet::Alphabet,
    engine::{self, general_purpose},
    Engine,
};
use permute::permutations_of;

fn main() -> anyhow::Result<()> {
    // Somewhere the html code (on VV -> Übersicht -> SB Mechanik) it says "Zurueck-Button, verschluesselt"

    // https://cryptii.com/
    let tmp = [
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        "abcdefghijklmnopqrstuvwxyz",
        "123456789",
        "0",
        "~",
        "-",
    ];

    for permutation in permutations_of(&tmp) {
        let alphabet = permutation.cloned().collect::<String>();
        let custom: Alphabet = base64::alphabet::Alphabet::new(&alphabet).unwrap();

        let engine: engine::GeneralPurpose =
            engine::GeneralPurpose::new(&custom, general_purpose::NO_PAD);

        let res = (|| -> anyhow::Result<()> {
            let file = File::open("base64.txt").unwrap();
            let lines = io::BufReader::new(file).lines();

            for line in lines {
                let _ = engine.decode(line?.trim_end_matches("_"))?;
            }
            Ok(())
        })();
        if res.is_ok() {
            println!("\n{}", alphabet);
            let file = File::open("base64.txt").unwrap();
            let lines = io::BufReader::new(file).lines();

            for line in lines {
                let result = engine.decode(line?.trim_end_matches("_"))?;
                stdout().write(&result)?;
            }
        }
    }

    Ok(())
}
