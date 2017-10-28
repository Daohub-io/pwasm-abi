#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), feature(alloc))]
#![feature(proc_macro)]
#![cfg(test)]

#[cfg(not(test))]
extern crate alloc;

#[cfg(not(test))]
use alloc::vec::Vec;

#[cfg(not(test))]
use alloc::borrow::Cow;
#[cfg(test)]
use std::borrow::Cow;

#[cfg(not(test))]
use core::cell::RefCell;
#[cfg(test)]
use std::cell::RefCell;

extern crate pwasm_abi;
extern crate parity_hash;
extern crate pwasm_abi_derive;
extern crate bigint;

use pwasm_abi_derive::eth_abi;

use bigint::U256;
use parity_hash::{H256, Address};

#[eth_abi(Endpoint, Client)]
pub trait TestContract {
	fn ctor(&mut self, _p: bool);

	fn baz(&mut self, _p1: u32, _p2: bool);
	fn boo(&mut self, _arg: u32) -> u32;
	fn sam(&mut self, _p1: Vec<u8>, _p2: bool, _p3: Vec<U256>);

	#[event]
	fn baz_fired(&mut self, indexed_p1: u32, p2: u32);
}

const PAYLOAD_SAMPLE_1: &[u8] = &[
	0xcd, 0xcd, 0x77, 0xc0,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
];

const PAYLOAD_SAMPLE_2: &[u8] = &[
	0xa5, 0x64, 0x3b, 0xf2,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x60,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xa0,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04,
	0x64, 0x61, 0x76, 0x65, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03,
];

const PAYLOAD_SAMPLE_3: &[u8] = &[
	0x5d, 0xda, 0xb4, 0xd4,
	0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x45,
];

#[cfg(test)]
thread_local!(pub static LAST_CALL: RefCell<Vec<u8>> = RefCell::new(Vec::new()));

#[cfg(test)]
fn call(_address: &Address, _value: U256, input: &[u8], _result: &mut [u8]) -> Result<(), ()> {
	LAST_CALL.with(|v| { *v.borrow_mut() = input.to_vec(); });
	Ok(())
}

#[cfg(test)]
fn log(_topics: &[H256], _data: &[u8]) {
}

#[test]
fn baz_dispatch() {
	#[derive(Default)]
	struct TestContractInstance {
		called: bool,
		called_wrong: bool,
	}

	impl TestContract for TestContractInstance {
		fn ctor(&mut self, _p1: bool) {
		}
		fn baz(&mut self, p1: u32, p2: bool) {
			assert_eq!(p1, 69);
			assert_eq!(p2, true);
			self.called = true;
		}
		fn boo(&mut self, _arg: u32) -> u32 {
			self.called_wrong = true;
			0
		}
		fn sam(&mut self, _p1: Vec<u8>, _p2: bool, _p3: Vec<U256>) {
			self.called_wrong = true;
		}
	}

	let mut endpoint = Endpoint::new(TestContractInstance::default());
	let result = endpoint.dispatch(PAYLOAD_SAMPLE_1);

	assert_eq!(result, Vec::new());

	assert!(endpoint.inner.called, "`baz` method was not invoked");
	assert!(!endpoint.inner.called_wrong, "wrong method was invoked");
}

#[test]
fn sam_dispatch() {
	#[derive(Default)]
	struct TestContractInstance {
		called: bool,
		called_wrong: bool,
	}

	impl TestContract for TestContractInstance {
		fn ctor(&mut self, _p1: bool) {
		}
		fn sam(&mut self, p1: Vec<u8>, p2: bool, p3: Vec<U256>) {
			assert_eq!(p1, vec![100, 97, 118, 101]);
			assert_eq!(p2, true);
			assert_eq!(p3, vec![
				[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01].into(),
				[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02].into(),
				[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03].into()
			]);
			self.called = true;
		}
		fn baz(&mut self, _p1: u32, _p2: bool) {
			self.called_wrong = true;
		}
		fn boo(&mut self, _arg: u32) -> u32 {
			self.called_wrong = true;
			0
		}
	}

	let mut endpoint = Endpoint::new(TestContractInstance::default());
	let result = endpoint.dispatch(PAYLOAD_SAMPLE_2);

	assert_eq!(result, Vec::new());

	assert!(endpoint.inner.called, "`sam` method was not invoked");
	assert!(!endpoint.inner.called_wrong, "wrong method was invoked");
}

#[test]
fn boo_dispatch() {
	#[derive(Default)]
	struct TestContractInstance {
		called: bool,
		called_wrong: bool,
	}

	impl TestContract for TestContractInstance {
		fn ctor(&mut self, _p1: bool) {
		}
		fn sam(&mut self, _p1: Vec<u8>, _p2: bool, _p3: Vec<U256>) {
			self.called_wrong = true;
		}
		fn baz(&mut self, _p1: u32, _p2: bool) {
			self.called_wrong = true;
		}
		fn boo(&mut self, arg: u32) -> u32 {
			self.called = true;
			assert_eq!(arg, 69);
			255
		}
	}

	let mut endpoint = Endpoint::new(TestContractInstance::default());
	let result = endpoint.dispatch(PAYLOAD_SAMPLE_3);

	assert_eq!(&result[28..32], &[0x00, 0x00, 0x00, 0xff]);

	assert!(endpoint.inner.called, "`boo` method was not invoked");
	assert!(!endpoint.inner.called_wrong, "wrong method was invoked");
}

#[test]
fn baz_call() {
	let mut client = Client::new(Address::zero());
	client.baz(69, true);
	LAST_CALL.with(|v| {
		let val: &[u8] = &v.borrow()[..];
		assert_eq!(val, PAYLOAD_SAMPLE_1);
	});
}