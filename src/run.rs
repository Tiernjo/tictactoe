extern mod rsfml;
use rsfml::graphics::{RectangleShape, RenderWindow, Color};

mod control;
mod grid;
mod window;


// Create a main loop
pub fn main_loop() {
	
	// Get Window
	let mut window = ::window::create();

	// Get Top Row of Grid
	let (top_left_bool, top_left) = ::grid::create_regions(0);
	let (top_mid_bool, top_mid) = ::grid::create_regions(1);
	let (top_right_bool, top_right) = ::grid::create_regions(2);
	// Get Mid Row of Grid
	let (mid_left_bool, mid_left) = ::grid::create_regions(3);
	let (mid_mid_bool, mid_mid) = ::grid::create_regions(4);
	let (mid_right_bool, mid_right) = ::grid::create_regions(5);
	// Get Bot Row of Grid
	let (bot_left_bool, bot_left) = ::grid::create_regions(6);
	let (bot_mid_bool, bot_mid) = ::grid::create_regions(7);
	let (bot_right_bool, bot_right) = ::grid::create_regions(8);

	// Get Lines
	let (left_line, right_line, top_line, bottom_line) = ::grid::create_lines();

	while window.is_open() {
		::control::input(&mut window);
		show(&mut window, 
			&top_left, &top_mid, &top_right, 
			&mid_left, &mid_mid, &mid_right, 
			&bot_left,	&bot_mid, &bot_right,
			&left_line, &right_line, &top_line, &bottom_line);
	}
}

// Clear buffer, draw items, show results
fn show(window: &mut RenderWindow, 
	top_left: &RectangleShape, top_mid: &RectangleShape, top_right: &RectangleShape, 
	mid_left: &RectangleShape, mid_mid: &RectangleShape, mid_right: &RectangleShape, 
	bot_left: &RectangleShape, bot_mid: &RectangleShape, bot_right: &RectangleShape,
	left_line: &RectangleShape, right_line: &RectangleShape, 
	top_line: &RectangleShape, bottom_line: &RectangleShape) {

	// Clean out window
	window.clear(&Color::white());
	// Draw grid_regions
	window.draw(top_left); window.draw(top_mid); window.draw(top_right);
	window.draw(mid_left); window.draw(mid_mid); window.draw(mid_right);
	window.draw(bot_left); window.draw(bot_mid); window.draw(bot_right);
	// Draw grid_lines
	window.draw(left_line); window.draw(right_line); 
	window.draw(top_line); window.draw(bottom_line);
	// Display window
	window.display()
}