use core::fmt::Debug;
use std::collections::BTreeMap;

use crate::system;


pub trait Config: system::Config {
    type Content: Debug + Ord;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>
}   

impl<T:Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new()
        }
    }
}