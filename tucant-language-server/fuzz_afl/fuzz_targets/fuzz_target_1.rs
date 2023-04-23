use afl::fuzz;

fn main() {
    fuzz!(|input: VecAction| { magic(input) });
}
