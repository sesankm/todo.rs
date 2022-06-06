use termion::color;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{Write, stdin, stdout};

struct Task {
	text: String,
	completed: bool
}

impl Task {
	fn new(name: String) -> Task {
		Task {
			text: name,
			completed: false
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
}

fn main() {
	let mut todo_list:TaskList = TaskList::new();

	todo_list.todos.push(Task::new("Task 1".to_string()));
	todo_list.todos.push(Task::new("Task 2".to_string()));
	todo_list.todos.push(Task::new("Task 3".to_string()));
	todo_list.todos.push(Task::new("Task 4".to_string()));

	todo_list.display();

	let stdin = stdin();
	let mut stdout = stdout().into_raw_mode().unwrap();

	for c in stdin.events() {
		match c.unwrap(){
			Event::Key(Key::Char('q')) => break,
			Event::Key(Key::Char('j')) => {
				if todo_list.selected + 1 < todo_list.todos.len() as i32 {
					todo_list.selected += 1;
				}
			},
			Event::Key(Key::Char('k')) => {
				if todo_list.selected - 1 >= 0 {
					todo_list.selected -= 1;
				}
			},
			Event::Key(Key::Char('d')) => {
				if todo_list.todos.len() == 0 {
					continue;
				}
				todo_list.todos.remove(todo_list.selected as usize);
				if todo_list.selected - 1 >= 0 {
					todo_list.selected -= 1;
				}
			},
			Event::Key(Key::Char('c')) => todo_list.todos[todo_list.selected as usize].toggle_status(),
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
