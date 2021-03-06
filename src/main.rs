// #![cfg_attr(feature="clippy", feature(plugin))]
// #![cfg_attr(feature="clippy", plugin(clippy))]

extern crate rodio; // https://github.com/tomaka/rodio/
extern crate ncurses; // https://github.com/jeaye/ncurses-rs
extern crate walkdir; // https://github.com/BurntSushi/walkdir

use std::fs::File;
use std::io::BufReader;
use std::ffi::OsStr;
use ncurses::*;
use walkdir::WalkDir;



fn main() {
    let mut playable_files = Vec::new();

    for entry in WalkDir::new(".") {
        let entry = entry.unwrap();
        let extension = entry.path().extension();
        if extension == Some(OsStr::new("ogg")) || extension == Some(OsStr::new("flac")) ||
            extension == Some(OsStr::new("wav"))
        {
            let path = entry.path().display().to_string();
            playable_files.push(path);
        } // is handled extension
    } // for entry in WalkDir::new(".")

    // init ncurses
    initscr();
    raw();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // invisible cursor
    noecho(); // dont echo keys

    // new window
    let window_x = playable_files.len() as i32;
    let mut window_y = 0;
    // find out longest song name
    for songname in &playable_files {
        let length = songname.len() as i32;
        if length > window_y {
            window_y = length;
        }
    }


    // pub fn newwin(lines: i32, cols: i32, y: i32, x: i32) -> WINDOW
    let w = newwin(window_x + 4, window_y + 5, 1, 0);
    box_(w, 0, 0);

    printw(
        "Rust Music Player. E to exit; P to add to playing queue; w, s to navigate",
    );

    refresh();

    /* let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    addstr(&format!(
        "window resolutions: \t max_x: {}, max_y: {}",
        max_x,
        max_y
    )); */

    highlight_nth(0, &playable_files, w);

    // use default_endpoint() once this works
    let endpoint = rodio::get_default_endpoint().unwrap();
    let sink = rodio::Sink::new(&endpoint);

    let mut index = 0 as i32;
    loop {
        // input loop
        let ch = getch();
        if ch == 'e' as i32 {
            // terminate
            endwin();
            break;
        } else if ch == 'w' as i32 {
            // move selection up
            if index == 0 {
                continue;
            }
            index -= 1;
            highlight_nth(index, &playable_files, w);
        } else if ch == 's' as i32 {
            // move selection down
            if index == (playable_files.len() - 1) as i32 {
                continue;
            }
            index += 1;
            highlight_nth(index, &playable_files, w);
        } else if ch == 'p' as i32 {
            // add to playing queue
            let filename = &playable_files[index as usize];
            let file = File::open(filename).unwrap();
            let audio_source = rodio::Decoder::new(BufReader::new(file)).unwrap();
            sink.append(audio_source);
            wrefresh(w);
        } else {
            // unrecognized key
            // nope
        }
        wrefresh(w);

        sink.sleep_until_end();

    } // loop
    // wait until everything is played

} // main


fn highlight_nth(index: i32, path_w_string_vec: &[String], window: WINDOW) {
    let maxlen = path_w_string_vec.len() as i32; // max list length
    for i in 0..maxlen {
        if i == index {
            attr_on(A_STANDOUT() as u32);
        } else {
            attr_off(A_STANDOUT() as u32);
        }
        let text = &path_w_string_vec[i as usize];
        mvprintw((i as i32) + 2, 3, text);
    }
    wrefresh(window);
}
