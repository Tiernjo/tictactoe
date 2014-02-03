extern mod rsfml;
use rsfml::window::{ContextSettings, VideoMode, Close};
use rsfml::graphics::{Color, RenderWindow};

// Make a window to draw on
pub fn create() -> RenderWindow{
	// Create a window
	let setting = ContextSettings::default();
	let mut window = match RenderWindow::new(VideoMode::new_init(900, 600, 32), "Tic-Tac-Toe", Close, &setting){
		Some(window) => window,
		None => fail!("Cannot create render window!")
	};
	window
}
