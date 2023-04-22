#![no_main]

use std::collections::HashMap;

use arbitrary::Arbitrary;
use libfuzzer_sys::fuzz_target;
use num_bigint::BigUint;
use tucant_language_server::evaluator::{Address, Allocator, BumpOnlyAddress, BumpOnlyAllocator};

#[derive(Debug, Arbitrary)]
enum Action {
    Allocate(u64),
    Set(usize, u64),
}

#[derive(Debug)]
pub struct VecAction(Vec<Action>);

impl<'a> Arbitrary<'a> for VecAction {
    fn arbitrary(u: &mut arbitrary::Unstructured<'a>) -> arbitrary::Result<Self> {
        let len = u.arbitrary_len::<Action>()?;
        let mut settable_addresses = Vec::new();

        let mut my_collection = Vec::with_capacity(len);
        for _ in 0..len {
            let element = match u.choose(&[1, 2])? {
                1 => {
                    let possibilities = u.arbitrary()?;
                    if possibilities != 0 {
                        settable_addresses.push(possibilities);
                    }
                    Action::Allocate(possibilities)
                }
                2 => {
                    let address_idx = u.choose_index(settable_addresses.len())?;
                    Action::Set(
                        address_idx,
                        u.int_in_range(0..=(settable_addresses[address_idx] - 1))?,
                    )
                }
                _ => unreachable!(),
            };
            my_collection.push(element);
        }

        Ok(VecAction(my_collection))
    }
}

fuzz_target!(|actions: VecAction| {
    println!("-------------------------------");
    let mut allocator = BumpOnlyAllocator::new();
    let mut settable_addresses = Vec::<BumpOnlyAddress>::new();
    let mut expected_values = HashMap::new();

    for action in actions.0 {
        match action {
            Action::Allocate(possibilities) => {
                //println!("possibilities {possibilities}");
                let address =
                    BumpOnlyAllocator::allocate(&mut allocator, BigUint::from(possibilities));
                //println!("address {:?}", address.clone());
                expected_values.insert(address.clone(), BigUint::from(0u8));
                if possibilities != 0 {
                    settable_addresses.push(address);
                }
            }
            Action::Set(address, value) => {
                settable_addresses[address].set(&mut allocator, BigUint::from(value));
                expected_values
                    .insert(settable_addresses[address].clone(), BigUint::from(value))
                    .unwrap();
            }
        }
    }

    for (k, v) in expected_values {
        assert_eq!(k.get(&mut allocator), v);
    }
});
