mod metric;


use std::thread;

use lazy_static::lazy_static;
use metric::Metrics;

lazy_static! {
    pub(crate) static ref METRICS: Metrics = Metrics::new(&[
        "topics",
        "clients",
        "peers",
        "broadcasts",
        "servers",
        "states",
        "subscribers"
    ]);
}

fn main() {
    METRICS.inc("topics");
    METRICS.inc("subscribers");

    println!("{:?}", METRICS.snapshot());

    let t1 = thread::spawn(|| {
        METRICS.inc("peers");
    });
    let t2 = thread::spawn(|| {
        METRICS.dec("subscribers");
    });

    t1.join().unwrap();
    t2.join().unwrap();
    println!("{:?}", METRICS.snapshot());
}