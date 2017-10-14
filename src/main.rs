//#![cfg_attr(feature="clippy", feature(plugin))]
//#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate rodio; // https://github.com/tomaka/rodio/
extern crate ncurses; // https://github.com/jeaye/ncurses-rs


use std::fs;
use std::fs::File;
use std::io::BufReader;
//use std::path::Path;
use std::ffi::OsStr;

use ncurses::*;


fn main() {
    let mut playable_files = Vec::new();
    // iterate over current directory
    if let Ok(files) = fs::read_dir("./") {
        for filename in files {
            if let Ok(filename) = filename {
                // filter playable files here

                if filename.path().extension() == Some(OsStr::new("ogg")) {
                    println!("Found file: {}", filename.path().display());
                    // collect filenames
                    playable_files.push(filename);
                } // is ogg?
            }
        } // for filename in files
    } // all files in cwd



    // init ncurses
    initscr();
    raw();
    // invisible cursor
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);




    printw("Testing ncurses screen, e to exit\n\n");

    // display files in ncurses
    for filename in &playable_files {
        //let  songentry = &song.path();
        addstr(&format!("Found file: {:?} \n", filename.path()));
    }

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

    // add all our found files to the queue
    for filename in playable_files {
        // @TODO this will probably blow up if we have a lot of large files
        let file = File::open(filename.path()).unwrap();
        let audio_source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        sink.append(audio_source);
    }
    sink.sleep_until_end(); // play everything until end

    println!("done");

}
