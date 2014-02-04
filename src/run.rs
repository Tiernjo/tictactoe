extern mod rsfml;
use rsfml::graphics::{Color, RectangleShape, RenderWindow};

mod control;
mod grid;
mod window;


// Create a main loop
pub fn main_loop() {
	
	// Get Window
	let mut window = ::window::create();

	// Get Lines
	let (left_line, right_line, top_line, bottom_line) = ::grid::create_lines();
	
	while window.is_open() {
		::control::input(&mut window);
		show(&mut window, 
			&left_line, &right_line, &top_line, &bottom_line);
	}
}

// Clear buffer, draw items, show results
fn show(window: &mut RenderWindow,
	left_line: &RectangleShape, right_line: &RectangleShape, 
	top_line: &RectangleShape, bottom_line: &RectangleShape) {

	// Clean out window
	window.clear(&Color::red());

	// Draw grid_lines
	window.draw(left_line); window.draw(right_line); 
	window.draw(top_line); window.draw(bottom_line);

	// Display window
	window.display()
}