use std::{cell::RefCell, rc::Rc};

use num_bigint::BigUint;

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
    possibilities: BigUint,
    address: BigUint,
}

impl Address for BumpOnlyAddress {
    fn set(&mut self, value: BigUint) {
        todo!()
    }

    fn get(&self) -> BigUint {
        todo!()
    }
}

#[test]
fn test_allocator() {
    let allocator = BumpOnlyAllocator::new();

    let addr0 = BumpOnlyAllocator::allocate(allocator, BigUint::from(8u8));

    addr0.set(5)
}
