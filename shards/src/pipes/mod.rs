//! Stardust's piping features.
//! defaults to unix pipes when not avalable.

mod fds;
mod streams;

pub use fds::open_cloexec;
