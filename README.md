# futures-stop

An example implementation of allowing [`futures::Stream`](http://alexcrichton.com/futures-rs/futures/stream/trait.Stream.html)s to be stopped remotely.

This is a very simple implementation which wraps a stream with another stream that will finish when it receives a message from a sender.

## Usage

```Rust
let (stream, handle) = futures_stop::convert_to_stoppable(stream);

// ...

handle.stop().wait().unwrap();
// or
handle.wait().stop();
```
