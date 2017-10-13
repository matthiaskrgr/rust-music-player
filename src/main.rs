// #![cfg_attr(feature="clippy", feature(plugin))]
// #![cfg_attr(feature="clippy", plugin(clippy))]

extern crate rodio; // https://github.com/tomaka/rodio/
extern crate ncurses; // https://github.com/jeaye/ncurses-rs


use std::fs::File;
use std::io::BufReader;

use ncurses::*;


fn main() {
    // init ncurses
    initscr();
    raw();
    // invisible cursor
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    printw("Testing ncurses screen, e to exit");

    mvprintw(LINES() - 1, 0, "e to exit");
    refresh();

    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);


    let mut ch = getch();
    // get user input (char), until we get an "e"
    while ch != 'e' as i32 {
        ch = getch();
    }
    endwin(); // close the ncurses window


    // play audio
    println!("starting audio");
    let endpoint = rodio::get_default_endpoint().unwrap();
    let sink = rodio::Sink::new(&endpoint);

    let file = File::open("test.ogg").unwrap();
    let audio_source = rodio::Decoder::new(BufReader::new(file)).unwrap();

    sink.append(audio_source);
    sink.sleep_until_end(); // play everything until end

    println!("done");

}
