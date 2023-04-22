#![no_main]

use libfuzzer_sys::fuzz_target;
use tucant_language_server::evaluator::BumpOnlyAllocator;

#[derive(Debug, Arbitrary)]
enum Action {
    Allocate(u64),
    Set(u64),
}

fuzz_target!(|data: &[Action]| {
    let allocator = BumpOnlyAllocator::new();
});
