extern mod rsfml;
use rsfml::graphics::{FloatRect, RectangleShape};

struct GridWidget;
impl GridWidget {
	fn block(&self) -> RectangleShape {
		let mut block = match RectangleShape::new() {
			Some(block)	=> block,
			None()		=> fail!(~"Error, creating Widget.")
		};

		block.set_size2f(100.0, 100.0);
		block
	}
}
pub fn create() -> (~[RectangleShape], ~[FloatRect]) {
	let base = GridWidget;
	let mut widgets_location = ~[];
	let mut widget_bounds = ~[];

	let mut i:int = 0;
	while i < 9 {
		widgets_location.push(base.block());
		i += 1;
	}

	let width:f32 = 600.0;
	let height:f32 = 600.0;

	widgets_location[0].set_position2f(height/3.0 * 0.05, height/3.0 * 0.10); 
	widgets_location[1].set_position2f( (width/3.0) + (width/3.0 * 0.05), height/3.0 * 0.10); 
	widgets_location[2].set_position2f((width*2.0/3.0) + (width/3.0 * 0.10), height/3.0 * 0.10);
	widgets_location[3].set_position2f(height/3.0 * 0.05, (height/3.0) + (height/3.0 * 0.05)); 
	widgets_location[4].set_position2f( (width/3.0) + (width/3.0 * 0.05), (height/3.0) + (height/3.0 * 0.05)); 
	widgets_location[5].set_position2f((width*2.0/3.0) + (width/3.0 * 0.10), (height/3.0) + (height/3.0 * 0.05));
	widgets_location[6].set_position2f(height/3.0 * 0.05, height*2.0/3.0); 
	widgets_location[7].set_position2f( (width/3.0) + (width/3.0 * 0.05), height*2.0/3.0); 
	widgets_location[8].set_position2f((width*2.0/3.0) + (width/3.0 * 0.10), height*2.0/3.0);

	let mut x:int = 0;
	while x < 9 {
		widget_bounds.push(widgets_location[x].get_global_bounds());
		x += 1;
	}
	(widgets_location, widget_bounds)
}