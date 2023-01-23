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

// how to do dynamic allocation?
pub trait Address {
    fn set(&mut self);
    fn get(&self);
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

pub struct BumpOnlyAddress {
    allocator: Rc<RefCell<BumpOnlyAllocator>>,
    inner: BigUint,
}

impl Address for BumpOnlyAddress {
    fn set(&mut self) {
        todo!()
    }

    fn get(&self) {
        todo!()
    }
}

impl Allocator for BumpOnlyAllocator {
    type AddressType = BumpOnlyAddress;

    fn allocate(this: Rc<RefCell<Self>>, possibilities: BigUint) -> Self::AddressType {
        let address = this.borrow().inner.clone();
        this.borrow_mut().inner *= possibilities;
        Self::AddressType {
            allocator: this,
            inner: address,
        }
    }
}

// cargo test -- --show-output evaluate
#[test]
const fn test_primitives() {}
