// #![cfg_attr(feature="clippy", feature(plugin))]
// #![cfg_attr(feature="clippy", plugin(clippy))]

extern crate rodio; // https://github.com/tomaka/rodio/

use std::fs::File;
use std::io::BufReader;
use rodio::Source;

use std::thread;
use std::time::Duration;

fn main() {

    println!("starting audio");
    let endpoint = rodio::get_default_endpoint().unwrap();
    let sink = rodio::Sink::new(&endpoint);

    let file = File::open("test.ogg").unwrap();
    let audio_source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    sink.append(audio_source);

    sink.sleep_until_end(); // play everything until end

    println!("done");

}
