use core::fmt::Debug;
use std::collections::BTreeMap;

use crate::{
	support::{self, DispatchResult},
	system,
};

pub trait Config: system::Config {
	type Content: Debug + Ord;
}

pub enum Call<T: Config> {
	CreateClaim { claim: T::Content },
	RevokeClaim { claim: T::Content },
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> support::Dispatch for Pallet<T> {
	type Call = Call<T>;
	type Caller = T::AccountId;

	fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
		match call {
			Call::CreateClaim { claim } => self.create_claim(caller, claim),
			Call::RevokeClaim { claim } => self.revoke_claim(caller, claim),
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { claims: BTreeMap::new() }
	}

	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
		return self.claims.get(claim);
	}

	pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		match self.get_claim(&claim) {
			Some(_) => Err("Claim already exists"),
			None => {
				self.claims.insert(claim, caller);
				Ok(())
			},
		}
	}

	pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		let claim_owner = self.get_claim(&claim).ok_or("Claim does not exist")?;

		if claim_owner != &caller {
			return Err("The claim does not belong to Caller");
		}

		self.claims.remove(&claim);
		Ok(())
	}
}

#[cfg(test)]
mod test {

	struct TestConfig;

	impl crate::system::Config for TestConfig {
		type Nonce = u32;
		type BlockNumber = u32;
		type AccountId = String;
	}

	impl super::Config for TestConfig {
		type Content = String;
	}

	#[test]
	fn basic_proof_of_existence() {
		let alice = "alice".to_string();
		let my_document: String = "my_document".to_string();
		let mut poe: super::Pallet<TestConfig> = super::Pallet::new();

		let _ = poe.create_claim(alice.clone(), my_document.clone());
		assert_eq!(poe.get_claim(&my_document), Some(&alice));
	}

	#[test]
	fn revoke_in_proof_of_existence() {
		let alice = "alice".to_string();
		let my_document: String = "my_document".to_string();
		let mut poe: super::Pallet<TestConfig> = super::Pallet::new();

		let _ = poe.create_claim(alice.clone(), my_document.clone());
		assert_eq!(poe.get_claim(&my_document), Some(&alice));

		let _ = poe.revoke_claim(alice.clone(), my_document.clone());
		assert_eq!(poe.get_claim(&my_document), None);
	}

	#[test]
	fn revoke_not_existing_claim() {
		let alice = "alice".to_string();
		let my_document: String = "my_document".to_string();
		let mut poe: super::Pallet<TestConfig> = super::Pallet::new();

		let result = poe.revoke_claim(alice.clone(), my_document.clone());
		assert_eq!(result, Err("Claim does not exist"));
	}

	#[test]
	fn revoke_not_owner_claim() {
		let alice = "alice".to_string();
		let bob: String = "bob".to_string();
		let my_document: String = "my_document".to_string();
		let mut poe: super::Pallet<TestConfig> = super::Pallet::new();

		let _ = poe.create_claim(alice.clone(), my_document.clone());

		let result: Result<(), &str> = poe.revoke_claim(bob.clone(), my_document.clone());
		assert_eq!(result, Err("The claim does not belong to Caller"));
	}
}
