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
	let (mid_left_bool, mid_left) = ::grid::create_grid(3);
	let (mid_mid_bool, mid_mid) = ::grid::create_grid(4);
	let (mid_right_bool, mid_right) = ::grid::create_grid(5);
	// Get Bot Row of Grid
	let (bot_left_bool, bot_left) = ::grid::create_grid(6);
	let (bot_mid_bool, bot_mid) = ::grid::create_grid(7);
	let (bot_right_bool, bot_right) = ::grid::create_grid(8);

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