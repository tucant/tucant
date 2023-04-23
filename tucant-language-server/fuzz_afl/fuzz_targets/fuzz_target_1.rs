use afl::fuzz;
use tucant_language_server_fuzz_common::{magic, VecAction};

fn main() {
    fuzz!(|input: VecAction| { magic(input) });
}
