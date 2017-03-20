use futures::Sink;
use futures::sink::{self, Send};
use futures::sync::mpsc::Sender;

/// Type alias for a [`Sender`] used for sending stop messages
///
/// [`Sender`]: ../futures/sync/mpsc/struct.Sender.html
pub type StopSender = Sender<()>;

/// A handle for a [`Stream`] which allows sending a stop message.
///
/// [`Stream`]: struct.Stream.html
#[derive(Clone)]
pub struct Handle
{
    /// The `Sender` used to send the stop message
    sender: StopSender,
}

impl Handle {
    /// Create a new handle with a specific [`StopSender`](type.StopSender.html).
    pub fn new(sender: StopSender) -> Handle {
        Handle { sender: sender }
    }

    /// Send a stop message.
    ///
    /// This is asynchronous; a future is returned and needs to be polled in order for the send to
    /// be completed. If you just want to send the message immediately and block until it finishes,
    /// use [`wait()`] to create the synchronous version and then call [`stop()`].
    ///
    /// [`wait()`]: #method.wait
    /// [`stop()`]: struct.Wait.html#method.stop
    pub fn stop(self) -> Send<StopSender> {
        self.sender.send(())
    }

    /// Creates a new object which will produce a synchronous handle.
    ///
    /// This is analogous to the [`Sink::wait()`] method.
    ///
    /// [`Sink::wait()`]: ../futures/sink/trait.Sink.html#method.wait
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Synchronously stop a stream
    /// handle.wait().stop();
    /// ```
    pub fn wait(self) -> Wait {
        Wait::new(self)
    }
}

/// A blocking, synchronous version of a [`Handle`](struct.Handle.html).
///
/// This is mainly for convenience, as we ignore the error when sending fails (the paired
/// stream has already been stopped), thus allowing slightly nicer code.
pub struct Wait {
    wait_sender: sink::Wait<StopSender>,
}

impl Wait {
    /// Create a new blocking version of a [`Handle`](struct.Handle.html).
    pub fn new(handle: Handle) -> Wait {
        Wait {
            wait_sender: handle.sender.wait(),
        }
    }

    /// Send a stop message to the matching [`Stream`] and block until the message has been
    /// successfully sent.
    ///
    /// [`Stream`]: struct.Stream.html
    pub fn stop(mut self) {
        let _ = self.wait_sender.send(());
    }
}
