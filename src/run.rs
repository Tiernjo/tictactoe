extern mod rsfml;
use rsfml::graphics::{Color, RectangleShape, RenderWindow};

mod control;
mod grid;
mod window;
mod widget;


// Create a main loop
pub fn main_loop() {
	
	// Get Window
	let mut window = ::window::create();

	// Get Lines
	let (left_line, right_line, top_line, bottom_line) = ::grid::create_lines();
	
	// Get Widgets
	let mut widgets = ::widget::create();
	let top_left = widgets.remove(0); let top_mid = widgets.remove(0); let top_right = widgets.remove(0);
	let mid_left = widgets.remove(0); let mid_mid = widgets.remove(0); let mid_right = widgets.remove(0);
	let bot_left = widgets.remove(0); let bot_mid = widgets.remove(0); let bot_right = widgets.remove(0);

	while window.is_open() {
		::control::input(&mut window);
		show(&mut window, 
			&left_line, &right_line, &top_line, &bottom_line,
			&top_left, &top_mid, &top_right,
			&mid_left, &mid_mid, &mid_right,
			&bot_left, &bot_mid, &bot_right);
	}
}

// Clear buffer, draw items, show results
fn show(window: &mut RenderWindow,
	left_line: &RectangleShape, right_line: &RectangleShape, 
	top_line: &RectangleShape, bottom_line: &RectangleShape,
	top_left: &RectangleShape, top_mid: &RectangleShape, top_right: &RectangleShape,
	mid_left: &RectangleShape, mid_mid: &RectangleShape, mid_right: &RectangleShape,
	bot_left: &RectangleShape, bot_mid: &RectangleShape, bot_right: &RectangleShape) {

	// Clean out window
	window.clear(&Color::red());

	// Draw grid_lines
	window.draw(left_line); window.draw(right_line); 
	window.draw(top_line); window.draw(bottom_line);

	// Draw widgets
	window.draw(top_left); window.draw(top_mid); window.draw(top_right);
	window.draw(mid_left); window.draw(mid_mid); window.draw(mid_right);
	window.draw(bot_left); window.draw(bot_mid); window.draw(bot_right);

	// Display window
	window.display()
}