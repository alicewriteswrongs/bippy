use std::sync::mpsc::{channel, Receiver};

/// Set up a ctrl-c interrupt 'listener' which can then be checked later on in e.g. a loop to see
/// if the user has tried to interrupt the program or not.
///
/// The message sent in the mpsc channel doesn't matter, so it's just a unit, we only care if a
/// message has actually been sent.
pub fn ctrlc_channel() -> Receiver<()> {
    let (sender, receiver) = channel::<()>();

    ctrlc::set_handler(move || {
        sender.send(()).expect("could not send into ctrl-c channel");
    })
    .expect("could not set up ctrl-c interrupt handler");

    receiver
}
