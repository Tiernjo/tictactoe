extern mod rsfml;
use rsfml::graphics::{Color, RectangleShape};
use rsfml::system::{Vector2f};

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
pub fn create() -> ~[RectangleShape] {
	let base = GridWidget;
	let mut widgets_location = ~[];
	let widget_type = [0, 1, 2];

	let mut i = 0;
	while i < 9 {
		widgets_location.push(base.block());
		i += 1;
	}

	let width:f32 = 900.0;
	let height:f32 = 600.0;

	widgets_location[0].set_position2f(0.0, 0.0); widgets_location[1].set_position2f(width/3.0, 0.0); widgets_location[2].set_position2f(width*2.0/3.0, 0.0);
	widgets_location[3].set_position2f(0.0, height/3.0); widgets_location[4].set_position2f(width/3.0, height/3.0); widgets_location[5].set_position2f(width*2.0/3.0, height/3.0);
	widgets_location[6].set_position2f(0.0, height*2.0/3.0); widgets_location[7].set_position2f(width/3.0, height*2.0/3.0); widgets_location[8].set_position2f(width*2.0/3.0, height*2.0/3.0);

	widgets_location
}