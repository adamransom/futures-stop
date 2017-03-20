use futures::{self, Async, Poll};
use futures::sync::mpsc::{self, Receiver};

use handle::Handle;

/// A stoppable version of the standard [`futures::Stream`]
///
/// This allows you force a stream to return `Async::Ready(None)` to indicate the stream has
/// finished.
///
/// [`futures::Stream`]: ../futures/stream/trait.Stream.html
pub struct Stream<S>
    where S: futures::Stream
{
    /// The stream being wrapped
    stream: S,
    /// The receiver used to receive the stop message
    rx: Receiver<()>,
}

impl<S> futures::Stream for Stream<S>
    where S: futures::Stream
{
    type Item = S::Item;
    type Error = S::Error;

    fn poll(&mut self) -> Poll<Option<S::Item>, S::Error> {
        match self.rx.poll() {
            // If we receive any message, or all senders are dropped (or an error occurs), we
            // return `Async::Ready(None)`, indicating this stream is finished
            Err(_) |
            Ok(Async::Ready(None)) |
            Ok(Async::Ready(Some(_))) => {
                Ok(Async::Ready(None))
            },
            // Otherwise, we call `poll` on the underlying stream
            Ok(Async::NotReady) => {
                self.stream.poll()
            },
        }
    }
}

/// Converts a regular [`futures::Stream`] into a [`Stream`] and [`Handle`] pair.
///
/// The [`Stream`] implements `futures::Stream`, so it can be used as usual, but a [`Handle`]
/// is also returned. This handle can be used to stop the stream at any point.
///
/// [`futures::Stream`]: ../futures/stream/trait.Stream.html
/// [`Stream`]: struct.Stream.html
/// [`Handle`]: struct.Handle.html
pub fn convert_to_stoppable<S>(stream: S) -> (Stream<S>, Handle)
    where S: futures::Stream
{
    // We don't really need a buffer bigger than this
    let (tx, rx) = mpsc::channel(0);

    (
        Stream { stream: stream, rx: rx },
        Handle::new(tx)
    )
}
