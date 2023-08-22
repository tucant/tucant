#![no_main]

use libfuzzer_sys::fuzz_target;
use tucant_language_server_fuzz_common::{magic, VecAction};

fuzz_target!(|input: VecAction| magic(input));
