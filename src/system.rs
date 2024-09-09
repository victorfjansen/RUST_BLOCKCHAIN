use std::{collections::BTreeMap, ops::AddAssign};

use num::{CheckedAdd, CheckedSub, One, Zero};

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + CheckedSub + CheckedAdd + Copy + AddAssign;
    type Nonce: Ord + Clone + Zero + One + CheckedSub + CheckedAdd + Copy;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	block_number: T::BlockNumber,
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> 
{
	pub fn new() -> Self {
		Self { block_number: T::BlockNumber::zero(), nonce: BTreeMap::new() }
	}

	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) {
		let new_block_number = self.block_number.checked_add(&T::BlockNumber::one()).unwrap_or(T::BlockNumber::zero());
		self.block_number = new_block_number;
	}

	pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let zero = T::Nonce::zero();
		let nonce = self.nonce.get(who).unwrap_or(&zero);
		let new_nonce = nonce.checked_add(&T::Nonce::one()).unwrap();

		self.nonce.insert(who.clone(), new_nonce);
	}

	pub fn get_nonce(&mut self, who: &T::AccountId) -> T::Nonce {
		*self.nonce.get(who).unwrap()
	}
}

#[cfg(test)]
mod test {

    struct TestConfig;

    impl super::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32; 
    }

	#[test]
	fn init_system() {
		let system: super::Pallet<TestConfig> = super::Pallet::new();
		assert_eq!(system.block_number(), 0);
	}

	#[test]
	fn inc_block_number() {
		let mut system: super::Pallet<TestConfig> = super::Pallet::new();
		system.inc_block_number();

		assert_eq!(system.block_number(), 1);
	}

	#[test]
	fn inc_nonce() {
		let alice = &"alice".to_string();

		let mut system: super::Pallet<TestConfig> = super::Pallet::new();
		system.inc_nonce(alice);

		assert_eq!(system.get_nonce(alice), 1);
	}
}
