//#![cfg_attr(feature="clippy", feature(plugin))]
//#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate rodio; // https://github.com/tomaka/rodio/
extern crate ncurses; // https://github.com/jeaye/ncurses-rs


use std::fs;
use std::fs::File;
use std::io::BufReader;
//use std::path::Path;
use std::ffi::OsStr;
//use std::{time, thread};
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
                    //println!("Found file: {}", filename.path().display());
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

/*    for i in &playable_files {
        println!("{}", i.path_string);
    } */

    // init ncurses
    initscr();
    raw();
    // invisible cursor
    //    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    // new window
    let w = newwin(10, 12, 1, 1);
    box_(w, 0, 0);
    let i = 0;
/*    for file in &playable_files {
        // highlight
        if i == 3 {
            wattron(w, A_STANDOUT as u32);
        } else {
            wattroff(w, A_STANDOUT as u32);
        }
        mvprintw(i + 1, 2, "menu entry\n");
        wrefresh(w);
    } // for
    wrefresh(w);
    */


    printw("Testing ncurses screen, e to exit\n\n");

    for file in &playable_files {
        addstr(&format!("{}\n", file.path_string));
    }

    mvprintw(LINES() - 1, 0, "e to exit");
    refresh();



    //thread::sleep(time::Duration::from_secs(4));

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

        let  ch = getch();
        //print!("{}\n", ch as i32);
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
            endwin();
            break;
        } else {
            // nope
        }
        wrefresh(w);
    }
    println!("playing  {}", play);
    let index = i;
    println!("playing audio");
    let endpoint = rodio::get_default_endpoint().unwrap(); // use default_endpoint() once this works
    let sink = rodio::Sink::new(&endpoint);

    let pathrev = &playable_files[index as usize];
    let path_string = &pathrev.path_string;
    let pathref = &pathrev.dir_entry;
    //let file = File::open( PathWithString[i as usize].path_string );

    println!("\n'{}'\n",  path_string);

    let file = File::open(path_string).unwrap(); 
    let audio_source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    sink.append(audio_source);

    sink.sleep_until_end(); // play everything in queu
    println!("done!");

} // main


fn highlight_nth(index: i32, path_w_string_vec: &Vec<PathWithString>, window: WINDOW) {
    for i in 0..5 {
        if i == index {
            attr_on(A_STANDOUT() as u32);
        } else {
            attr_off(A_STANDOUT() as u32);
        }
        let text = &path_w_string_vec[i as usize].path_string;
        mvprintw((i as i32) + 1, 2, text);
    }
    wrefresh(window);
}
