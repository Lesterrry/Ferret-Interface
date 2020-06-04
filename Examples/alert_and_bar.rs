mod Misc;
mod Alert;
mod ProgressBar;

fn main() {
	Misc::overwrite();
	loop{
		let mut progress_bar = ProgressBar::new("LOADING...".to_string(), 800, FerretColor::Blue);
		for _i in 0..800{
			thread::sleep(time::Duration::from_millis(10));
			progress_bar.increase();
		}
		match Alert::invoke("LOADING FAULT".to_string(), ("RELOAD".to_string(), "EXIT".to_string()), FerretColor::Red){
			AlertButton::Left => (),
			AlertButton::Right => break,
		}
	}
}
