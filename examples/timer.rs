extern crate futures;
extern crate futures_stop;
extern crate tokio_timer;

use std::time::*;

use futures::*;
use futures_stop::convert_to_stoppable;
use tokio_timer::*;

fn main() {
    // Create a timer and use `interval()` to create a `Stream`
    let timer = Timer::default();
    let timer_stream = timer.interval(Duration::from_secs(2));
    // Convert the stream into it's stoppable counterpart, getting a handle to the stream at the
    // same time
    let (timer, handle) = convert_to_stoppable(timer_stream);

    // Spawn a new thread, continuously printing when the timer fires and blocking until the stream
    // ends. Usually, this stream would never end and `println!("Done Thread")` would never be hit.
    let thread = std::thread::spawn(move || {
        timer.for_each(|_| {
            println!("Timer Fired");
            Ok(())
        }).wait().unwrap();

        println!("Done Thread");
    });

    // Wait 10 seconds before asking the timer stream to stop
    std::thread::sleep(Duration::from_secs(10));
    handle.wait().stop();

    // Join the thread so we get to see the `Done Thread` message
    thread.join().unwrap();

    println!("Done Main");
}
