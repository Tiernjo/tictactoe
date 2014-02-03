extern mod rsfml;
use rsfml::window::{event, keyboard};
use rsfml::graphics::{RectangleShape, RenderWindow, Color};

mod window;

mod grid;

// Create a main loop
pub fn main_loop() {
	
	// Get Window
	let mut window = ::window::create();

	// Get Top Row of Grid
	let (top_left_bool, top_left) = ::grid::create_grid(0);
	let (top_mid_bool, top_mid) = ::grid::create_grid(1);
	let (top_right_bool, top_right) = ::grid::create_grid(2);
	// Get Mid Row of Grid
	let mut mid_left = ::grid::mid_left();
	let mut mid_mid = ::grid::mid_mid();
	let mut mid_right = ::grid::mid_right();
	// Get Bot Row of Grid
	let mut bot_left = ::grid::bot_left();
	let mut bot_mid = ::grid::bot_mid();
	let mut bot_right = ::grid::bot_right();

	while window.is_open() {
		controls(&mut window);
		show(&mut window, 
			&top_left, &top_mid, &top_right, 
			&mid_left, &mid_mid, &mid_right, 
			&bot_left,	&bot_mid, &bot_right);
	}
}

// When close button is clicked, closes window
// When escape key is pressed, closes window
// Else fucks off
fn controls(window: &mut RenderWindow) {
	loop{
		match window.poll_event() {
			event::Closed => window.close(),
			event::KeyPressed{code, ..} => match code {
				keyboard::Escape => {window.close(); break},
				_ => {}
			},
			event::NoEvent => break,
			_ => {}
		}
	}
}

// Clear buffer, draw items, show results
fn show(window: &mut RenderWindow, 
	top_left: &RectangleShape, top_mid: &RectangleShape, top_right: &RectangleShape, 
	mid_left: &RectangleShape, mid_mid: &RectangleShape, mid_right: &RectangleShape, 
	bot_left: &RectangleShape, bot_mid: &RectangleShape, bot_right: &RectangleShape) {

	window.clear(&Color::white());
	window.draw(top_left);
	window.draw(top_mid);
	window.draw(top_right);
	window.draw(mid_left);
	window.draw(mid_mid);
	window.draw(mid_right);
	window.draw(bot_left);
	window.draw(bot_mid);
	window.draw(bot_right);
	window.display()
}