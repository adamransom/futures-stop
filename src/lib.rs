//! An example implementation of allowing [`futures::Stream`]s to be stopped remotely.
//!
//! This is a very simple implementation which wraps a stream with another stream that will finish
//! when it receives a message from a sender.
//!
//! [`futures::Stream`]: ../future/stream/trait.Stream.html
//!
//! # Examples
//!
//! ```ignore
//! let (stream, handle) = futures_stop::convert_to_stoppable(stream);
//!
//! // ...
//!
//! handle.stop().wait().unwrap();
//! // or
//! handle.wait().stop();
//! ```
extern crate futures;

mod handle;
mod stream;

pub use self::handle::*;
pub use self::stream::*;
