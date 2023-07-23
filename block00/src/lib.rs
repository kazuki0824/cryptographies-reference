#![cfg_attr(not(test), no_std)]
mod soft_backend;

#[cfg(test)]
mod tests;

pub use soft_backend::expand_00;
pub use soft_backend::feistel;
pub use soft_backend::round_function;
