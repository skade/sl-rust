extern crate sl;
extern crate libc;
extern crate ncurses;
extern crate getopts;

use getopts::{optflag,getopts};
use std::os;

use sl::{Train};
use sl::d51::SL;
use sl::c51::C51;
use sl::logo::Logo;

trait Render : Train + Copy {
  fn render(&self, x: i32) {
    let mut len = 0 as i32;
    let y = ncurses::LINES / 2;
    let body_iter = self.body().iter();
    let wheelset_iter = self.wheelset(x as uint).iter();
    let iter = body_iter.chain(wheelset_iter);
    let (_, hint) = iter.size_hint();
    let height = match hint {
      Some(s) => s,
      None => panic!("this really shouldn't happen")
    };
    let offset = (height / 2) as i32;
    for (index, line) in iter.rev().enumerate() {
      if line.len() as i32 > len {
        len = line.len() as i32;
      }
      self.render_line((y + offset) - index as i32, x, *line);
    }
    if let Some(tender) = self.tender() {
      let mut new_len = 0 as i32;
      for (index, line) in tender.iter().rev().enumerate() {
        if len + line.len() as i32 > new_len {
          new_len = len + line.len() as i32;
        }
        self.render_line((y + offset) - index as i32, x + len, *line);
      }
      len = new_len;
    }
    if let Some(wagon) = self.wagon() {
      for _ in range(0,self.wagons()) {
        let mut new_len = 0 as i32;
        for (index, line) in wagon.iter().rev().enumerate() {
          if len + line.len() as i32 > new_len {
            new_len = len + line.len() as i32;
          }
          self.render_line((y + offset) - index as i32, x + len, *line);
        }
        len = new_len;
      }
    }
  }

  fn render_line(&self, y: i32, x: i32, line: &str) {
    let paint_len = (ncurses::COLS - x) as uint;
    if paint_len < line.len() {
      ncurses::mvaddstr(y, x, line[0..paint_len]);
    } else if x < 0 {
      if -x < line.len() as i32 {
        ncurses::mvaddstr(y, 0, line[-x as uint..line.len()]);
      }
    } else {
      ncurses::mvaddstr(y, x, line);
    }
  }
}

impl Render for SL { }
impl Render for C51 { }
impl Render for Logo { }

fn main() {
  use libc::funcs::posix01::signal::signal;
  use libc::funcs::posix88::unistd::usleep;
  use libc::SIGINT;
  use libc::consts::os::posix01::SIG_IGN;

  let args: Vec<String> = os::args();

  let opts = [
    optflag("l", "", "logo"),
    optflag("c", "", "C51"),
    optflag("a", "", "reserved for future use"),
    optflag("f", "", "reserved for future use"),
  ];

  let matches = match getopts(args.tail(), &opts) {
    Ok(m) => { m }
    Err(f) => { panic!(f.to_string()) }
  };

  let train: Box<Render> = if matches.opt_present("l") {
                             box Logo
                           } else if matches.opt_present("c") {
                             box C51
                           } else {
                             box SL
                           };


  ncurses::initscr();
  unsafe { signal(SIGINT, SIG_IGN); }

  ncurses::noecho();
  ncurses::curs_set(ncurses::CURSOR_INVISIBLE);
  ncurses::nodelay(ncurses::stdscr, true);
  ncurses::leaveok(ncurses::stdscr, true);
  ncurses::scrollok(ncurses::stdscr, false);

  for x in range(-85, ncurses::COLS).rev() {
    ncurses::clear();
    train.render(x);
    ncurses::getch();
    ncurses::refresh();
    unsafe { usleep(40000); }
  }
  ncurses::endwin();
}
