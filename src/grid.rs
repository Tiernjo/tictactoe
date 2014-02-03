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
	let width:f32 = 800.0;
	let height:f32 = 600.0;
	let zero_zero:Vector2f = Vector2f::new(0.0, 0.0);
	let third_zero:Vector2f = Vector2f::new(width/3.0, 0.0);
	let two_zero:Vector2f = Vector2f::new(width * 2.0 / 3.0, 0.0);

	let third_third:Vector2f = Vector2f::new(width/3.0, height/3.0);

	let grid_bool:GridSquare = GridSquare{is_x: ~false, is_empty: ~true};
	let mut block = grid_bool.block();

	match which {
		0	=>	{block.set_position(&zero_zero); block.set_size(&third_third); block.set_fill_color(&Color::red());}
		1	=>	{block.set_position(&third_zero); block.set_size(&third_third); block.set_fill_color(&Color::blue());}
		2	=>	{block.set_position(&two_zero); block.set_size(&third_third); block.set_fill_color(&Color::magenta());}
		3	=>	{}
		4	=>	{}
		5	=>	{}
		6   =>	{}
		7	=>	{}
		8	=>	{}
		_	=>	fail!("Error, cannot initialize {}'s block.", which + 1)
	}

	(grid_bool, block)
}

/////////////////////////////////////////////////////////////////////////
// Top Row of Grid
/////////////////////////////////////////////////////////////////////////
pub fn top_left() -> (RectangleShape, GridSquare) {
	let width:f32 = 800.;
	let height:f32 = 600.;
	
	let mut grid_bool:GridSquare = GridSquare{is_x: ~false, is_empty: ~true};
	let mut top_left = grid_bool.block();

	top_left.set_origin2f(0.0, 0.0);
	top_left.set_size2f(width/3.0, height/3.0);
	top_left.set_fill_color(&Color::transparent());
	(top_left, grid_bool)
}
pub fn top_mid() -> RectangleShape {
	let width:f32 = 800.0;
	let height:f32 = 600.0;

	let grid_bool:GridSquare = GridSquare{is_x: ~false, is_empty: ~true};
	let mut top_mid = grid_bool.block();

	top_mid.set_position2f(width/3.0, 0.0);
	top_mid.set_size2f(width/3.0 , height/3.0);
	top_mid.set_fill_color(&Color::transparent());
	top_mid
}
pub fn top_right() -> RectangleShape {
	let width:f32 = 800.0;
	let height:f32 = 600.0;

	let grid_bool:GridSquare = GridSquare{is_x: ~false, is_empty: ~true};
	let mut top_right = grid_bool.block();

	top_right.set_position2f(width * 2.0/3.0, 0.0);
	top_right.set_size2f(width/3.0 , height/3.0);
	top_right.set_fill_color(&Color::transparent());
	top_right
}

///////////////////////////////////////////////////////////////////////////
// Middle Row of Grid
///////////////////////////////////////////////////////////////////////////
pub fn mid_left() -> RectangleShape{
	let width:f32 = 800.;
	let height:f32 = 600.;
	
	let grid_bool:GridSquare = GridSquare{is_x: ~false, is_empty: ~true};
	let mut mid_left = grid_bool.block();

	mid_left.set_position2f(0.0, height/3.0);
	mid_left.set_size2f(width/3.0, height/3.0);
	mid_left.set_fill_color(&Color::transparent());
	mid_left
}
pub fn mid_mid() -> RectangleShape {
	let width:f32 = 800.0;
	let height:f32 = 600.0;

	let grid_bool:GridSquare = GridSquare{is_x: ~false, is_empty: ~true};
	let mut mid_mid = grid_bool.block();

	mid_mid.set_position2f(width/3.0, height/3.0);
	mid_mid.set_size2f(width/3.0 , height/3.0);
	mid_mid.set_fill_color(&Color::transparent());
	mid_mid
}
pub fn mid_right() -> RectangleShape {
	let width:f32 = 800.0;
	let height:f32 = 600.0;

	let grid_bool:GridSquare = GridSquare{is_x: ~false, is_empty: ~true};
	let mut mid_right = grid_bool.block();

	mid_right.set_position2f(width * 2.0/3.0, height/3.0);
	mid_right.set_size2f(width/3.0 , height/3.0);
	mid_right.set_fill_color(&Color::transparent());
	mid_right
}

/////////////////////////////////////////////////////////////////////////////
// Bot row of Grid
/////////////////////////////////////////////////////////////////////////////
pub fn bot_left() -> RectangleShape{
	let width:f32 = 800.;
	let height:f32 = 600.;
	
	let grid_bool:GridSquare = GridSquare{is_x: ~false, is_empty: ~true};
	let mut bot_left = grid_bool.block();

	bot_left.set_position2f(0.0, height * 2.0/3.0);
	bot_left.set_size2f(width/3.0, height/3.0);
	bot_left.set_fill_color(&Color::transparent());
	bot_left
}
pub fn bot_mid() -> RectangleShape {
	let width:f32 = 800.0;
	let height:f32 = 600.0;

	let grid_bool:GridSquare = GridSquare{is_x: ~false, is_empty: ~true};
	let mut bot_mid = grid_bool.block();

	bot_mid.set_position2f(width/3.0, height * 2.0/3.0);
	bot_mid.set_size2f(width/3.0 , height/3.0);
	bot_mid.set_fill_color(&Color::transparent());
	bot_mid
}
pub fn bot_right() -> RectangleShape {
	let width:f32 = 800.0;
	let height:f32 = 600.0;

	let grid_bool:GridSquare = GridSquare{is_x: ~false, is_empty: ~true};
	let mut bot_right = grid_bool.block();

	bot_right.set_position2f(width * 2.0/3.0, height * 2.0/3.0);
	bot_right.set_size2f(width/3.0 , height/3.0);
	bot_right.set_fill_color(&Color::transparent());
	bot_right
}