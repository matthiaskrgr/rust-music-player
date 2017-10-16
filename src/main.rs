//#![cfg_attr(feature="clippy", feature(plugin))]
//#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate rodio; // https://github.com/tomaka/rodio/
extern crate ncurses; // https://github.com/jeaye/ncurses-rs


use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::ffi::OsStr;
use ncurses::*;

struct PathWithString {
    dir_entry: std::fs::DirEntry,
    path_string: String,
}

fn main() {

    let mut playable_files = Vec::new();
    // iterate over current directory
    if let Ok(files) = fs::read_dir("./") {
        for filename in files {
            if let Ok(filename) = filename {

                // filter playable files here
                if filename.path().extension() == Some(OsStr::new("ogg")) {
                    // collect filenames
                    let path = filename.path().display().to_string();
                    let path_w_string = PathWithString {
                        dir_entry: filename,
                        path_string: path,
                    };
                    playable_files.push(path_w_string);

                } // is ogg?
            }
        } // for filename in files
    } // all files in cwd

    // init ncurses
    initscr();
    raw();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE); // invisible cursor

    // new window
    let window_x = playable_files.len() as i32;
    let mut window_y = 0;
    // find out longest song name
    for songname in &playable_files {
        let length = songname.path_string.len() as i32;
        if length > window_y {
            window_y = length;
        }
    }


    // pub fn newwin(lines: i32, cols: i32, y: i32, x: i32) -> WINDOW
    let w = newwin(window_x + 4, window_y + 5, 1, 0); // 25: get max song length
    box_(w, 0, 0);

    printw("Rust Music Player. E to exit.");

    for file in &playable_files {
        addstr(&format!("{}\n", file.path_string));
    }
    refresh();
    noecho(); // dont echo keys

    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    addstr(&format!(
        "window resolutions: \t max_x: {}, max_y: {}",
        max_x,
        max_y
    ));

    highlight_nth(0, &playable_files, w);
    let mut i = 0 as i32;
    let mut play = "";
    loop {
        let ch = getch();
        if ch == 'e' as i32 {
            // terminate
            endwin();
            break;
        } else if ch == 'w' as i32 {
            if i == 0 {
                continue;
            }
            i -= 1;
            highlight_nth(i, &playable_files, w);
            wrefresh(w);
        } else if ch == 's' as i32 {
            if i == (playable_files.len() - 1) as i32 {
                continue;
            }
            i += 1;
            highlight_nth(i, &playable_files, w);

            wrefresh(w);
        } else if ch == 'p' as i32 {
            // play
            play = &playable_files[i as usize].path_string;
        //endwin();
        //break;
        } else {
            // nope
        }
        wrefresh(w);
        let index = i;
        // use default_endpoint() once this works
        let endpoint = rodio::get_default_endpoint().unwrap();
        let sink = rodio::Sink::new(&endpoint);

        let filename = &playable_files[index as usize].path_string;
        let file = File::open(filename).unwrap();
        let audio_source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        sink.append(audio_source);
        sink.sleep_until_end(); // play everything in queu
    }

} // main


fn highlight_nth(index: i32, path_w_string_vec: &Vec<PathWithString>, window: WINDOW) {
    let maxlen = path_w_string_vec.len() as i32; // max list length
    for i in 0..maxlen {
        if i == index {
            attr_on(A_STANDOUT() as u32);
        } else {
            attr_off(A_STANDOUT() as u32);
        }
        let text = &path_w_string_vec[i as usize].path_string;
        mvprintw((i as i32) + 2, 3, text);
    }
    wrefresh(window);
}
