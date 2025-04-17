use ref_cell::*;

fn main() {
    // initialize the worker
    let logger = Worker::new(1);

    // initialize the tracker, with the max number of
    // called references as 10
    let track = Tracker::new(&logger, 10);

    let _a = logger.track_value.clone();    // |\
    let _a1 = logger.track_value.clone();   // | -> increase the Rc to 4 references
    let _a2 = logger.track_value.clone();   // |/

    // take a peek of how much we already used from our quota
    track.peek(&logger.track_value);

    let _b = logger.track_value.clone();  // |\
    let _b1 = logger.track_value.clone(); // |  -> increase the Rc to 8 references
    let _b2 = logger.track_value.clone(); // | /
    let _b3 = logger.track_value.clone(); // |/

    // this will set the value and making a verification of
    // how much we already used of our quota
    track.set_value(&logger.track_value);

    let _c = logger.track_value.clone(); // | -> increase the Rc to 9 references

    // this will set the value and making a verification of
    // how much we already used of our quota
    track.set_value(&logger.track_value);

    let _c1 = logger.track_value.clone(); // | -> increase the Rc to 10 references, this will be the limit

    track.set_value(&logger.track_value);

    for (k ,v) in logger.mapped_messages.into_inner() {
        println!("{:?}", (k ,v));
    }
    println!("{:?}", logger.all_messages.into_inner());
}
