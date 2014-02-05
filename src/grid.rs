extern mod rsfml;
use rsfml::graphics::{Color, RectangleShape};
use rsfml::system::{Vector2f};

struct GridLine;
impl GridLine {
	fn block(&self) -> RectangleShape {
		let block = match RectangleShape::new() {
			Some(block)	=> block,
			None()		=> fail!(~"Error, creating block")
		};
		block
	}
}

pub fn create(which:&str) -> (RectangleShape, RectangleShape, RectangleShape, RectangleShape){
	// window width and height
	let width:f32 = 600.0;
	let height:f32 = 600.0;

	// Vector2f of the sizes of the lines
	let mut vertical_line_size = Vector2f::new(height/3.0 * 0.10, width); let mut horizontal_line_size = Vector2f::new(width, height /3.0 * 0.10);

	// Vector2f of the starting positions of lines
	let mut left_position = Vector2f::new(0.0,0.0); let mut right_position = Vector2f::new(0.0,0.0);	
	let mut top_position = Vector2f::new(0.0,0.0); let mut bottom_position = Vector2f::new(0.0,0.0);

	let base = GridLine;
	let mut left = base.block();let mut right = base.block();let mut top = base.block(); let mut bottom = base.block();
	
	if which == "grid" {
		left_position = Vector2f::new(width / 3.0 * 0.95, 0.0); right_position = Vector2f::new(width * 2.0 / 3.0, 0.0);
		top_position = Vector2f::new(0.0, height / 3.0 * 0.95); bottom_position = Vector2f::new(0.0, height * 2.0 / 3.0 * 0.95);
	} else if which == "border" {
		// left, top, and right work fine
		// bottom is who knows where
		left_position = Vector2f::new(0.0, 0.0); right_position = Vector2f::new(width - (width/3.0 * 0.05), 0.0);
		top_position = Vector2f::new(0.0, 0.0); bottom_position = Vector2f::new(0.0 , height - (height/3.0 * 0.10));

		vertical_line_size = Vector2f::new(height/3.0 * 0.05, width); let horizontal_line_size = Vector2f::new(width, height /3.0 * 0.05);
	} else {

	}
	// Set position, size and color of "lines"
	left.set_position(&left_position); left.set_size(&vertical_line_size); left.set_fill_color(&Color::black());
	right.set_position(&right_position); right.set_size(&vertical_line_size); right.set_fill_color(&Color::black());
	top.set_position(&top_position); top.set_size(&horizontal_line_size); top.set_fill_color(&Color::black());
	bottom.set_position(&bottom_position); bottom.set_size(&horizontal_line_size); bottom.set_fill_color(&Color::black());
	
	(left, right, top, bottom)
}



