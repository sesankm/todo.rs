use std::fs;
use std::io::{Write, stdin, stdout};
use std::str::Split;

use termion::color;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

#[derive(Debug)]
struct Task {
	text: String,
	completed: bool
}

fn read_file() -> TaskList {
	let mut tl: TaskList = TaskList::new();

	let contents = fs::read_to_string(".tasks").expect("Error reading file.");
	let tasks_raw: Split<&str> = contents.split("\n");

	for task in tasks_raw {
		let mut task_details: Split<&str> = task.split(",");

		let text = task_details.next();
		let completion = task_details.next();

		if text == None || completion == None {
			break;
		}

		tl.add_task(Task::new(text.unwrap().to_string(), completion.unwrap().to_string() == "1".to_string()));
	}
	tl
}

impl Task {
	fn new(name: String, completed: bool) -> Task {
		Task {
			text: name,
			completed: completed
		}
	}

	fn toggle_status(&mut self) {
		self.completed = !self.completed;
	}

	fn display(&self, row:i32, selected:bool) {
		match self.completed {
			true => print!("[X] "),
			false => print!("[ ] ")
		}
		match selected {
			true => print!("{}{}", color::Fg(color::Black), color::Bg(color::White)),
			false => print!("{}{}", color::Fg(color::White), color::Bg(color::Black))
		}
		println!("{}{}{}", self.text, termion::style::Reset, termion::cursor::Goto(1, row as u16))
	}

}

struct TaskList {
	selected: i32,
	todos: Vec<Task>
}

impl TaskList {
	fn new() -> TaskList {
		TaskList {
			selected: 0,
			todos: Vec::new()
		}
	}

	fn display(&self) {
		print!("{}", termion::clear::All);
		print!("{}", termion::cursor::Hide);
		print!("{}", termion::cursor::Goto(1, 1));

		if self.todos.len() as i32 == 0 {
			println!("No todo items.");
		}
		for i in 0..self.todos.len() as i32 {
			self.todos[i as usize].display(i + 1, i == self.selected);
		}
	}

	fn add_task(&mut self, task: Task) {
		self.todos.push(task);
	}

	fn inc(&mut self) {
		if self.selected + 1 < self.todos.len() as i32 {
			self.selected += 1;
		}
	}

	fn dec(&mut self){
		if self.selected - 1 >= 0 {
			self.selected -= 1;
		}
	}

	fn remove_selected(&mut self){
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
}

fn get_input() -> String {
	let stdin = stdin();
	let mut foo = String::new();
	println!("\n\n{}{}New task name{}: ", color::Fg(color::Black), color::Bg(color::White), termion::style::Reset);
	let _ = stdin.read_line(&mut foo);
	foo
}

fn main() {
	let stdin = stdin();
	let mut stdout = stdout().into_raw_mode().unwrap();
	let mut todo_list = read_file();

	todo_list.display();

	for c in stdin.events() {
		match c.unwrap(){
			Event::Key(Key::Char('q')) => break,
			Event::Key(Key::Char('j')) => todo_list.inc(),
			Event::Key(Key::Char('k')) => todo_list.dec(),
			Event::Key(Key::Char('d')) => todo_list.remove_selected(),
			Event::Key(Key::Char('c')) => todo_list.todos[todo_list.selected as usize].toggle_status(),
			Event::Key(Key::Char('a')) => {
				let _ = RawTerminal::suspend_raw_mode(&stdout);
				todo_list.add_task(Task::new(get_input(), false));
				let _ = RawTerminal::activate_raw_mode(&stdout);
			},
			_ => {}
		}
		stdout.flush().unwrap();
		todo_list.display();
	}

	print!("{}", termion::clear::All);
	print!("{}", termion::cursor::Hide);
	print!("{}", termion::cursor::Goto(1, 1));
	println!("{}", termion::cursor::Show);
}
