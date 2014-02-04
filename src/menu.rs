extern mod rsfml;
use rsfml::graphics::{Color, Font, RectangleShape, Text};
use rsfml::system::{Vector2f, Vector2u};

pub fn create(window_size:Vector2u) -> (RectangleShape, Text) {
	let menu_x = window_size.x as f32;
	let menu_y = window_size.y as f32;
	let menu_vec = Vector2f::new(menu_x, menu_y); 

	let mut menu:RectangleShape = match RectangleShape::new() {
		Some(menu)	=> menu,
		None()		=> fail!(~"Error, creating menu.")
	};

	let menu_font:Font = match Font::new_from_file("../resources/Minecrafter_3.ttf") {
		Some(menu_font)	=> menu_font,
		None()			=> fail!(~"Error, creating menu font.")
	};

	let mut menu_text:Text = match Text::new() {
		Some(menu_text)	=> menu_text,
		None()			=> fail!(~"Error, creating menu text.")
	};

	menu.set_size(&menu_vec);
	menu.set_fill_color(&Color::red());

	menu_text.set_font(&menu_font);
	menu_text.set_character_size(40);
	menu_text.set_position2f(0.0, 0.0);
	menu_text.set_color(&Color::black());
	menu_text.set_string("Welcome to Tic-Tac-Toe");

	(menu, menu_text)
}