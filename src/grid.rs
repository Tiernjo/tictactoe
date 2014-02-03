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

pub fn create_grid(which: int) -> (GridSquare, RectangleShape){
	let width:f32 = 900.0;
	let height:f32 = 600.0;
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

	let grid_bool:GridSquare = GridSquare{is_x: ~false, is_empty: ~true};
	let mut block = grid_bool.block();

	match which {
		0	=>	{block.set_position(&zero_zero); block.set_size(&third_third); block.set_fill_color(&Color::red());}	// top_left
		1	=>	{block.set_position(&third_zero); block.set_size(&third_third); block.set_fill_color(&Color::black());}	// top_mid
		2	=>	{block.set_position(&two_zero); block.set_size(&third_third); block.set_fill_color(&Color::red());}	// top_right
		3	=>	{block.set_position(&zero_third); block.set_size(&third_third); block.set_fill_color(&Color::black());}	// mid_left
		4	=>	{block.set_position(&third_third); block.set_size(&third_third); block.set_fill_color(&Color::red());}	// mid_mid
		5	=>	{block.set_position(&two_third); block.set_size(&third_third); block.set_fill_color(&Color::black());}		// mid_right
		6   =>	{block.set_position(&zero_two); block.set_size(&third_third); block.set_fill_color(&Color::red());}
		7	=>	{block.set_position(&third_two); block.set_size(&third_third); block.set_fill_color(&Color::black());}
		8	=>	{block.set_position(&two_two); block.set_size(&third_third); block.set_fill_color(&Color::red());}
		_	=>	fail!("Error, cannot initialize {}'s block.", which + 1)
	}

	(grid_bool, block)
}