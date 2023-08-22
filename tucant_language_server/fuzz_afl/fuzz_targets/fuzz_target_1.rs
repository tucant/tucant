#[cfg(fuzzing)]
use afl::fuzz;
#[cfg(fuzzing)]
use tucant_language_server_fuzz_common::{magic, VecAction};

fn main() {
    #[cfg(fuzzing)]
    fuzz!(|input: VecAction| { magic(input) });
}
