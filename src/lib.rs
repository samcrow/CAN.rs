
#![deny(
    missing_docs,
    trivial_casts,
    missing_debug_implementations
)]

//!
//! This library provides types and traits for CAN sending and receiving. It does not contain
//! implementations for sending and receiving.
//!

#[macro_use]
extern crate quick_error;

mod message;
pub use message::*;

/// A trait for things that can send and receive CAN messages
pub trait Can {
    /// The error type for this implementation
    type Err: ::std::error::Error;
    /// Sends a message
    fn send(&mut self, message: Message) -> Result<(), Self::Err>;
    /// Receives a message
    fn receive(&mut self) -> Result<Message, Self::Err>;
}
