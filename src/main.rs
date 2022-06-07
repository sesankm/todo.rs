use std::io::{stdin, stdout, Write};
use std::env;

use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::{IntoRawMode};

mod task;

fn main() {
	let args: Vec<String> = env::args().collect();
	let mut todo_list = task::read_file();
	if args.len() > 1 {
		if args.contains(&"d".to_string()) || args.contains(&"-d".to_string()) {
			for i in 0..todo_list.todos.len() as i32 {
				todo_list.todos[i as usize].display(-1, false);
			}
		}
	}
	else {
		let stdin = stdin();
		let mut stdout = stdout().into_raw_mode().unwrap();
		println!("{}", todo_list);

		for c in stdin.events() {
			match c.unwrap(){
				Event::Key(Key::Char('q')) => break,
				Event::Key(Key::Char('j')) => todo_list.inc(),
				Event::Key(Key::Char('k')) => todo_list.dec(),
				Event::Key(Key::Char('d')) => todo_list.del(),
				Event::Key(Key::Char('c')) => todo_list.mark(),
				Event::Key(Key::Char('a')) => todo_list.add(&mut stdout),
				_ => {}
			}
			stdout.flush().unwrap();
			println!("{}", todo_list);
		}

		print!("{}", termion::clear::All);
		print!("{}", termion::cursor::Hide);
		print!("{}", termion::cursor::Goto(1, 1));
		println!("{}", termion::cursor::Show);
		todo_list.dump();
	}
}
