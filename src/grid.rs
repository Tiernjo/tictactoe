extern mod rsfml;
use rsfml::graphics::{Color, RectangleShape};
use rsfml::system::{Vector2f};

struct GridSquare {
	is_empty: ~bool,
	is_x: ~bool
}

impl GridSquare {
	fn block (&self) -> RectangleShape {
		let block = match RectangleShape::new() {
			Some(block)	=> block,
			None() 		=> fail!(~"Error, creating block."),
		};
		block
	}
}



pub fn create_regions(which: int) -> (GridSquare, RectangleShape){
	let width:f32 = 900.0;
	let height:f32 = 600.0;

	// Vector2f of each needed coordinate point set
	let zero_zero:Vector2f = Vector2f::new(0.0, 0.0);
	let third_zero:Vector2f = Vector2f::new(width/3.0, 0.0);
	let two_zero:Vector2f = Vector2f::new(width * 2.0 / 3.0, 0.0);
	let zero_third:Vector2f = Vector2f::new(0.0, height/3.0);
	let two_third:Vector2f = Vector2f::new(width * 2.0/3.0, height/3.0);
	let zero_two:Vector2f = Vector2f::new(0.0, height * 2.0 / 3.0);
	let third_two:Vector2f = Vector2f::new(width / 3.0, height * 2.0 / 3.0);
	let two_two:Vector2f = Vector2f::new(width * 2.0 / 3.0, height * 2.0 / 3.0);

	// Size of each grid
	let third_third:Vector2f = Vector2f::new(width/3.0, height/3.0);

	// Create each region
	let grid_bool:GridSquare = GridSquare{is_x: ~false, is_empty: ~true};
	let mut block = grid_bool.block();

	// Assign position, size, and color of block depending on which block is being made
	match which {
		0	=>	{block.set_position(&zero_zero); block.set_size(&third_third); block.set_fill_color(&Color::red());}	// top_left
		1	=>	{block.set_position(&third_zero); block.set_size(&third_third); block.set_fill_color(&Color::white());}	// top_mid
		2	=>	{block.set_position(&two_zero); block.set_size(&third_third); block.set_fill_color(&Color::red());}		// top_right
		3	=>	{block.set_position(&zero_third); block.set_size(&third_third); block.set_fill_color(&Color::white());}	// mid_left
		4	=>	{block.set_position(&third_third); block.set_size(&third_third); block.set_fill_color(&Color::red());}	// mid_mid
		5	=>	{block.set_position(&two_third); block.set_size(&third_third); block.set_fill_color(&Color::white());}	// mid_right
		6   =>	{block.set_position(&zero_two); block.set_size(&third_third); block.set_fill_color(&Color::red());}		// bot_left
		7	=>	{block.set_position(&third_two); block.set_size(&third_third); block.set_fill_color(&Color::white());}	// bot_mid
		8	=>	{block.set_position(&two_two); block.set_size(&third_third); block.set_fill_color(&Color::red());}		// bot_right
		_	=>	fail!("Error, cannot initialize {}'s block.", which + 1)
	}

	(grid_bool, block)
}

pub fn create_lines() -> (RectangleShape, RectangleShape, RectangleShape, RectangleShape){
	// window width and height
	let width:f32 = 900.0;
	let height:f32 = 600.0;

	// Vector2f of the sizes of the lines
	let vertical_line_size = Vector2f::new(height/3.0 * 0.10, height);
	let horizontal_line_size = Vector2f::new(width, height /3.0 * 0.10);

	// Vector2f of the starting positions of lines
	let left_position = Vector2f::new(width / 3.0 * 0.95, 0.0);
	let right_position = Vector2f::new(width * 2.0 / 3.0, 0.0);	
	let top_position = Vector2f::new(0.0, height / 3.0 * 0.95);
	let bottom_position = Vector2f::new(0.0, height * 2.0 / 3.0 * 0.95);

	// Create Lines
	let mut left = match RectangleShape::new() {
		Some(left)	=> left,
		None()		=> fail!(~"Error, cannot create left line.")
	};
	let mut right = match RectangleShape::new() {
		Some(right)	=> right,
		None()		=> fail!(~"Error, cannot create right line.")
	};
	let mut top = match RectangleShape::new() {
		Some(top)	=> top,
		None()		=> fail!(~"Error, cannot create top line.")
	};
	let mut bottom = match RectangleShape::new() {
		Some(bottom)	=> bottom,
		None()		=> fail!(~"Error, cannot create bottom line.")
	};

	// Set position, size and color of "lines"
	left.set_position(&left_position); left.set_size(&vertical_line_size); left.set_fill_color(&Color::black());
	right.set_position(&right_position); right.set_size(&vertical_line_size); right.set_fill_color(&Color::black());
	top.set_position(&top_position); top.set_size(&horizontal_line_size); top.set_fill_color(&Color::black());
	bottom.set_position(&bottom_position); bottom.set_size(&horizontal_line_size); bottom.set_fill_color(&Color::black());

	(left, right, top, bottom)
}

