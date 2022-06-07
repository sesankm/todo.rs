use std::fs;
use std::fmt::{Display, Formatter, Result};
use std::fs::File;
use std::io::{stdin, Write};
use std::str::Split;

use termion::color;
use termion::raw::RawTerminal;

pub fn read_file() -> TaskList {
	let mut tl: TaskList = TaskList::new();
	let contents = fs::read_to_string("/home/sesank/.tasks");
	match contents {
		Err(_) => return tl,
		_ => {}
	}

	let contents = contents.unwrap();
	let tasks_raw: Split<&str> = contents.split("\n");
	for task in tasks_raw {
		let mut task_details: Split<&str> = task.split(",");
		let text = task_details.next();
		let completion = task_details.next();
		if text == None || completion == None {
			break;
		}
		tl._add(text.unwrap().to_string(), completion.unwrap().to_string() == "1".to_string());
	}
	tl
}

pub fn show_controls(num_items: i32) {
	let mut row = 10;
	if num_items > 8 {
		row = num_items + 4;
	}

	println!("{}{} <a> add task \t <c> mark/unmark \t <q> quit {}",
			 termion::cursor::Goto(1, row as u16),
			 color::Fg(color::Yellow),
			 termion::style::Reset);
	println!("{}{} <q> quit \t <d> delete task \t <j>/<k> navigate {} ",
			 termion::cursor::Goto(1, (row + 1) as u16),
			 color::Fg(color::Yellow),
			 termion::style::Reset);
}

pub struct Task {
	text: String,
	completed: bool
}

impl Task {
	pub fn new(name: String, completed: bool) -> Task {
		Task {
			text: name,
			completed: completed
		}
	}

	pub fn mark(&mut self) {
		self.completed = !self.completed;
	}

	pub fn display(&self, row:i32, selected:bool) {
		match self.completed {
			true => print!("[X] "),
			false => print!("[ ] ")
		}
		if row == -1 {
			println!("{}", self.text);
		}
		else {
			match selected {
				true => print!("{}{}", color::Fg(color::Black), color::Bg(color::White)),
				false => print!("{}{}", color::Fg(color::White), color::Bg(color::Black))
			}
			println!("{}{}{}", self.text, termion::style::Reset, termion::cursor::Goto(1, (row + 1) as u16))
		}
	}
}

pub struct TaskList {
	pub selected: i32,
	pub todos: Vec<Task>
}

impl TaskList {
	pub fn new() -> TaskList {
		TaskList {
			selected: 0,
			todos: Vec::new()
		}
	}

	pub fn _add(&mut self, name: String, status: bool) {
		self.todos.push(Task::new(name, status));
	}

	pub fn add<W: std::io::Write>(&mut self, stdout: &mut RawTerminal<W>) {
		let stdin = stdin();
		let _ = RawTerminal::suspend_raw_mode(&stdout);

		println!("\n\n{}{}New task name{}: ", color::Fg(color::Black), color::Bg(color::White), termion::style::Reset);

		let mut new_task = String::new();
		let _ = stdin.read_line(&mut new_task);
		let _ = RawTerminal::activate_raw_mode(&stdout);
		self._add(new_task, false);
	}

	pub fn del(&mut self){
		if self.todos.len() > 0 {
			self.todos.remove(self.selected as usize);
		}
		if self.todos.len() == 0 {
			return;
		}
		else if (self.selected as usize) >= self.todos.len() {
			self.selected = (self.todos.len() - 1) as i32;
		}
	}

	pub fn inc(&mut self) {
		if self.todos.len() as i32 == 0 {
			return;
		}
		if self.selected != (self.todos.len() - 1) as i32 {
			self.selected += 1;
		}
	}

	pub fn dec(&mut self){
		if self.selected != 0 {
			self.selected -= 1;
		}
	}

	pub fn mark(&mut self){
		self.todos[self.selected as usize].mark();
	}

	pub fn dump(&self){
		let mut file = File::create("/home/sesank/.tasks").unwrap();
		for task in &self.todos {
			let _ = file.write_fmt(format_args!("{},{}\n", task.text.replace("\n", ""), task.completed as i32));
		}
	}
}

impl Display for TaskList {
	fn fmt(&self, _f: &mut Formatter<'_>) -> Result {
		print!("{}", termion::clear::All);
		print!("{}", termion::cursor::Hide);
		print!("{}", termion::cursor::Goto(1, 1));

		if self.todos.len() as i32 == 0 {
			println!("No todo items.");
		}
		for i in 0..self.todos.len() as i32 {
			self.todos[i as usize].display(i, i == self.selected);
		}

		show_controls(self.todos.len() as i32);
		Ok(())
	}
}
