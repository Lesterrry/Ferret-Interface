extern crate termion;
use std::io::{Write, stdout};
use termion::{terminal_size, color, style, raw::IntoRawMode, input::TermRead};
use std::{thread, time};

pub struct Misc;
impl Misc {
	#[allow(dead_code)]
	fn overwrite(){
		let stdout = stdout();
		let mut stdout = stdout.lock().into_raw_mode().unwrap();
		write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();

		let pres = terminal_size().unwrap().0;
		for _i in 0..pres{
			for _j in 0..terminal_size().unwrap().1{
				print!(" ");
			}
		}
		write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
	}
}

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
enum FerretColor{
	Red,
	Blue,
	Gray
}
enum AlertButton{
	Right,
	Left
}
pub struct Alert;
impl Alert {
	#[allow(dead_code)]
	fn invoke(message: String, buttons: (String, String), color: FerretColor) -> AlertButton {
		Alert::draw(message.clone(), buttons.clone(), (false, false), (false, false), color.clone());
		let mut set = 0;
		let stdout = stdout();
		let stdout = stdout.into_raw_mode().unwrap();
		let mut stdin = termion::async_stdin().keys();
		loop {
			let input = stdin.next();
			if let Some(Ok(key)) = input {
				match key {
					termion::event::Key::Char('1') | termion::event::Key::Char('Y') | termion::event::Key::Char('y') => 
						{ Alert::draw(message.clone(), buttons.clone(), (true, false), (true, false), color.clone()); 
							thread::sleep(time::Duration::from_millis(200));
							Misc::overwrite(); return AlertButton::Left },
					termion::event::Key::Char('2') | termion::event::Key::Char('N') | termion::event::Key::Char('n') =>
						{ Alert::draw(message.clone(), buttons.clone(), (false, true), (false, true), color.clone());
							thread::sleep(time::Duration::from_millis(200));
							Misc::overwrite(); return AlertButton::Right },
					termion::event::Key::Right => set = 2,
					termion::event::Key::Left => set = 1,
					termion::event::Key::Char('\n') | termion::event::Key::Char(' ') => {
						match set {
							1 => { Alert::draw(message.clone(), buttons.clone(), (true, false), (true, false), color.clone()); 
							thread::sleep(time::Duration::from_millis(200));
							Misc::overwrite(); return AlertButton::Left },
							2 => {Alert::draw(message.clone(), buttons.clone(), (false, true), (false, true), color.clone()); 
							thread::sleep(time::Duration::from_millis(200));
							Misc::overwrite(); return AlertButton::Right },
							0 | _ => ()
						}
					},
					_ => ()
				}
				stdout.lock().flush().unwrap();
				match set {
					1 => Alert::draw(message.clone(), buttons.clone(), (true, false), (false, false), color.clone()),
					2 => Alert::draw(message.clone(), buttons.clone(), (false, true), (false, false), color.clone()),
					0 | _ => Alert::draw(message.clone(), buttons.clone(), (false, false), (false, false), color.clone()),
				}
			}
			thread::sleep(time::Duration::from_millis(30));
		}
	}

	fn draw(message: String, buttons: (String, String), selection: (bool, bool), pushing:(bool, bool), color: FerretColor){
		if message.len() > 38 || buttons.0.len() > 10 || buttons.1.len() > 10 {panic!("Message is too long")}
		let offset = (terminal_size().unwrap().0 / 2 - 20, terminal_size().unwrap().1 / 2 - 4);
		let offset_message = 20 - message.len() / 2;
		let offset_buttons = (9 - buttons.0.len() / 2, 31 - buttons.1.len() / 2);
		let mut colors_sel = [0,0];
		if selection.0 == true { colors_sel[0] = 1};
		if selection.1 == true { colors_sel[0] = 1};
		print!("{}{}{}", termion::cursor::Goto(offset.0, offset.1), 
			if color == FerretColor::Red { "\x1b[48;5;88m" } 
			else if color == FerretColor::Gray { "\x1b[48;5;238m" }
			else {"\x1b[48;5;24m"}, "\x1b[38;5;243m");
		for i in 0..8{
			match i {
			1 => print!("┌─────────────────────────────────────┐" ),
			2 => 
				{
					let mut j = 0;
					loop{
						if j == offset_message {
							print!("{}{}{}", color::Fg(color::White), message, "\x1b[38;5;243m");
							j += message.len();
						}
						else {print!(" ")}
						j += 1;
						if j > 39 {break;}
					}
				},
			4 => 	if pushing == (false, false){
						print!("  {}┌{}──────────{}┐       {}┌{}──────────{}┐{}  ",
						if selection.0 == true {"\x1b[37m"}
						else {"\x1b[38;5;243m"},
						if selection.0 == true {"\x1b[37m┬"}
						else {"\x1b[38;5;243m─"},
						if selection.0 == true {"\x1b[37m┬"}
						else {"\x1b[38;5;243m─"},
						if selection.1 == true {"\x1b[37m"}
						else {"\x1b[38;5;243m"},
						if selection.1 == true {"\x1b[37m┬"}
						else {"\x1b[38;5;243m─"},
						if selection.1 == true {"\x1b[37m┬"}
						else {"\x1b[38;5;243m─"},
						"\x1b[38;5;243m")
					}else {
						if pushing.0 == true { print!("{}                       ┌┬──────────┬┐  ", "\x1b[38;5;243m") }
						else {print!("{}  ┌┬──────────┬┐                       ", "\x1b[38;5;243m" )}
					},
			5 => 
				if pushing == (false, false){
					let mut j = 0;
					loop{
						if j == offset_buttons.0 {
							print!("{}{}{}", color::Fg(color::White), buttons.0, color::Fg(color::LightBlack));
							j += buttons.0.len();
						}
						else if j == offset_buttons.1 {
							print!("{}{}{}", color::Fg(color::White), buttons.1, color::Fg(color::LightBlack));
							j += buttons.1.len() - 1;
						}
						else {print!(" ")}
						j += 1;
						if j > 39 {break;}
					}
				}else{
					if pushing.1 == true { print!("{}                       ┌┬──────────┬┐  ", "\x1b[37m") }
					else {print!("{}  ┌┬──────────┬┐                       ", "\x1b[37m" )}
				},
			6 => print!("  {}└{}──────────{}┘       {}└{}──────────{}┘{}  ",
						if selection.0 == true {"\x1b[37m"}
						else {"\x1b[38;5;243m"}, 
						if selection.0 == true {"\x1b[37m┴"}
						else {"\x1b[38;5;243m─"},
						if selection.0 == true {"\x1b[37m┴"}
						else {"\x1b[38;5;243m─"},
						if selection.1 == true {"\x1b[37m"}
						else {"\x1b[38;5;243m"},
						if selection.1 == true {"\x1b[37m┴"}
						else {"\x1b[38;5;243m─"},
						if selection.1 == true {"\x1b[37m┴"}
						else {"\x1b[38;5;243m─"},
						color::Fg(color::LightBlack)),
			7 => print!("└─────────────────────────────────────┘" ),
			_ => print!("                                       " ),
			}
			print!("{}", termion::cursor::Goto(offset.0, offset.1 + i));
		}
		println!("{}", style::Reset);
	}
}

pub struct ProgressBar{
	max: usize,
    progression: usize,
    message: String,
    color: FerretColor
}
impl ProgressBar{
	fn new(message: String, max: usize, color: FerretColor) -> Self{
		ProgressBar{
			max,
			progression: 0,
			message,
			color
		}
	}
	fn invoke(&mut self){
		ProgressBar::draw(self.message.clone(), self.progression.to_string() + "/" + &self.max.to_string(), self.progression / 27, self.color.clone());
	}
	fn increase(&mut self){
		self.progression += 1;
		ProgressBar::invoke(self);
	}
	fn draw(message: String, description: String, position: usize, color: FerretColor){
		if message.len() > 38  {panic!("Message is too long")}
		let offset = (terminal_size().unwrap().0 / 2 - 20, terminal_size().unwrap().1 / 2 - 4);
		let offset_message = 20 - message.len() / 2;
		let offset_description = 20 - description.len() / 2;
		print!("{}{}{}", termion::cursor::Goto(offset.0, offset.1), 
			if color == FerretColor::Red { "\x1b[48;5;88m" } 
			else if color == FerretColor::Gray { "\x1b[48;5;238m" }
			else {"\x1b[48;5;24m"}, "\x1b[38;5;243m");
		for i in 0..8{
			match i {
				1 => print!("┌─────────────────────────────────────┐" ),
				2 => 
				{
					let mut j = 0;
					loop{
						if j == offset_message {
							print!("{}{}{}", color::Fg(color::White), message, "\x1b[38;5;243m");
							j += message.len();
						}
						else {print!(" ")}
						j += 1;
						if j > 39 {break;}
					}
				},
				3 => 
				{
					let mut j = 0;
					loop{
						if j == offset_description {
							print!("{}{}{}", color::Fg(color::White), description, "\x1b[38;5;243m");
							j += description.len();
						}
						else {print!(" ")}
						j += 1;
						if j > 39 {break;}
					}
				},
				5 => {
					for _i in 0..5{
						print!(" {}", "\x1b[37m");
					}
					for _i in 0..position{
						print!("━");
					}
					for _i in 0..34 - position{
						print!(" {}", "\x1b[38;5;243m");
					}
				},
				6 => print!("    └─────────────────────────────┘    " ),
				7 => print!("└─────────────────────────────────────┘" ),
				_ => print!("                                       " ),
			}
			print!("{}", termion::cursor::Goto(offset.0, offset.1 + i));
		}
		println!("{}", style::Reset);
	}
}
