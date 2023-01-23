use std::{cell::RefCell, rc::Rc};

use num_bigint::BigUint;
use num_integer::Integer;

use crate::parser::Ast;

pub trait AnyData {
    fn get_possibilities(&self) -> BigUint;
    fn get_value(&self) -> BigUint;
}

pub struct Boolean {
    inner: BigUint,
}

impl AnyData for Boolean {
    fn get_possibilities(&self) -> BigUint {
        BigUint::from(2u8)
    }

    fn get_value(&self) -> BigUint {
        self.inner.clone()
    }
}

impl AnyData for Ast {
    fn get_possibilities(&self) -> BigUint {
        todo!()
    }

    fn get_value(&self) -> BigUint {
        todo!()
    }
}

pub trait Address {
    fn set(&mut self, value: BigUint);
    fn get(&self) -> BigUint;
}

pub trait Allocator
where
    Self::AddressType: Address,
{
    type AddressType;

    fn allocate(this: Rc<RefCell<Self>>, possibilities: BigUint) -> Self::AddressType;
}

pub struct BumpOnlyAllocator {
    inner: BigUint,
}

impl BumpOnlyAllocator {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            inner: BigUint::from(0u8),
        }))
    }
}

impl Allocator for BumpOnlyAllocator {
    type AddressType = BumpOnlyAddress;

    fn allocate(this: Rc<RefCell<Self>>, possibilities: BigUint) -> Self::AddressType {
        let address = this.borrow().inner.clone();
        this.borrow_mut().inner *= &possibilities;
        Self::AddressType {
            allocator: this,
            possibilities,
            address,
        }
    }
}

pub struct BumpOnlyAddress {
    allocator: Rc<RefCell<BumpOnlyAllocator>>,
    #[cfg(debug_assertions)]
    possibilities: BigUint,
    address: BigUint,
}

impl Address for BumpOnlyAddress {
    fn set(&mut self, value: BigUint) {
        debug_assert!(value < self.possibilities);

        let value = &self.allocator.borrow().inner;
        let (base_quotient, base_remainder) = value.div_rem(&self.address);

        let (remaining_quotient, our_value) = base_remainder.div_rem(&self.possibilities);

        let base_remainder = remaining_quotient * &self.possibilities + value;
        let value = base_quotient * &self.address + base_remainder;
        self.allocator.borrow_mut().inner = value;
    }

    fn get(&self) -> BigUint {
        let value = &self.allocator.borrow().inner;
        let our_value = (value / &self.address) % &self.possibilities;
        our_value
    }
}

#[test]
fn test_allocator() {
    let allocator = BumpOnlyAllocator::new();

    let mut addr0 = BumpOnlyAllocator::allocate(allocator.clone(), BigUint::from(7u8));
    addr0.set(BigUint::from(5u8));
    assert_eq!(addr0.get(), BigUint::from(5u8));

    let mut addr1 = BumpOnlyAllocator::allocate(allocator, BigUint::from(11u8));
    addr1.set(BigUint::from(3u8));
    assert_eq!(addr1.get(), BigUint::from(3u8));

    assert_eq!(addr0.get(), BigUint::from(5u8));
}
