//! Method signature uses lifetime.

#![feature(const_vec_new)]
use near_bindgen::near_bindgen;
use borsh::{BorshDeserialize, BorshSerialize};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
struct Ident {
    value: u32,
}

#[near_bindgen]
impl Ident {
    pub fn is_ident<'a>(&self, other: &'a u32) -> Option<&'a u32> {
        if *other == self.value {
            Some(other)
        } else {
            None
        }
    }
}

fn main() {}
