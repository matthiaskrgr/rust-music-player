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

    /*
    let mut playable_files = Vec::new();
    // iterate over current directory
    if let Ok(files) = fs::read_dir("./") {
        for filename in files {
            if let Ok(filename) = filename {
                // filter playable files here

                if filename.path().extension() == Some(OsStr::new("ogg")) {
                    //println!("Found file: {}", filename.path().display());
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
    //    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    // new window
    let w = newwin(10, 12, 1, 1);
    box_(w, 0, 0);

    for i in 0..5 {
        if i == 4 {
            // highlighting
            wattron(w, A_STANDOUT as u32);
        } else {
            wattroff(w, A_STANDOUT as u32);
        }
        mvprintw(i+1, 2, "menu entry\n");
        wrefresh(w);
    }

    wrefresh(w);

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

    addstr(&format!(
        "window resolutions: \t max_x: {}, max_y: {}",
        max_x,
        max_y
    ));

    let mut ch = getch();
    // get user input (char), until we get an "e"
    while ch != 'e' as i32 {
        ch = getch();
    }
    endwin(); // close the ncurses window


    // play audio
    println!("starting audio");
    let endpoint = rodio::get_default_endpoint().unwrap(); // use default_endpoint() once this works
    let sink = rodio::Sink::new(&endpoint);

    // add all our found files to the queue
    for filename in &playable_files {
        // @TODO this will probably blow up if we have a lot of large files
        let file = File::open(filename.path()).unwrap();
        let audio_source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        sink.append(audio_source);
    }
    sink.sleep_until_end(); // play everything until end

    println!("done");

*/

    let mut menus = Vec::new();
    menus.push("menu one");
    menus.push("menu two");
    menus.push("menu three");
    menus.push("menu four");
    menus.push("menu five");
    let menus = menus;
    initscr();
    //    raw();
    let w = newwin(10, 12, 1, 1);
    box_(w, 0, 0);

    highlightNth(0, &menus, w);
    wrefresh(w);

    noecho(); // dont print typed stuff on screen
    keypad(w, true); // enable keyboard input


    let mut i = 0 as i32;
    loop {

        let mut ch = getch();
        //print!("{}\n", ch as i32);
        if ch == 'e' as i32 {
            // terminate
            endwin();
            break;
        } else if ch == 'w' as i32 { // KEY_DOWN
            i -=1;
            highlightNth(i, &menus, w);
            wrefresh(w);
        } else if ch == 's' as i32 { // KEY_UP
            i +=1;
            highlightNth(i, &menus, w);
            wrefresh(w);
        } else {
            // nope
        }
        wrefresh(w);
    }


}

fn highlightNth(index: i32, textVec: &Vec<&str>, window:WINDOW) {
    for i in 0..5 {
        if i == index {
            attr_on(A_STANDOUT() as u32);
        } else {
            attr_off(A_STANDOUT() as u32);
        }
        let text = &textVec[i as usize];
        mvprintw((i as i32) + 1, 2, text);
    }
    wrefresh(window);
}
