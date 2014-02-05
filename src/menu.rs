extern mod rsfml;
extern mod std;

use rsfml::graphics::{Color, Font, RectangleShape};
use rsfml::graphics::rc::Text;
use rsfml::system::{Vector2f, Vector2u};

use std::cell::RefCell;
use std::rc::Rc;

struct Menu;
impl Menu {
	fn create_block(&self) -> RectangleShape {
		let mut block = match RectangleShape::new() {
			Some(block)	=>	block,
			None()			=>	fail!(~"Error, creating button.")
		};
		block
	}
	fn create_font(&self) -> Font {
		let font:Font = match Font::new_from_file("../resources/Minecrafter_3.ttf") {
			Some(font)	=>	font,
			None()		=>	fail!(~"Error, creating font."),
		};

		font
	}
	fn create_text(&self) -> Text {
		let text:Text = match Text::new() {
			Some(text)	=>	text,
			None()		=> fail!(~"Error, creating text.")
		};
		text
	}
}

pub fn create(window_size:Vector2u) -> (RectangleShape, Text, Text) {
	let menu_x = window_size.x as f32;
	let menu_y = window_size.y as f32;
	let menu_vec = Vector2f::new(menu_x, menu_y);
	let half_width:f32 = 600.0/2.0;

	// Create Main menu
	let main_menu:Menu = Menu;
	let menu_window = main_menu.create_block();	// whole window
	let menu_font = main_menu.create_font();	//font
	//set up font
	let menu_ref_cell = RefCell::new(menu_font); let menu_Rc = Rc::new(menu_ref_cell);
	let mut menu_title = main_menu.create_text();	// text

	let title_option:Menu = Menu;
	let option_font = title_option.create_font();
	let option_ref_cell = RefCell::new(option_font); let option_Rc = Rc::new(option_ref_cell);
	let mut menu_option = title_option.create_text();

	// Set menut text properties
	menu_title.set_font(menu_Rc);	// assign font
	menu_title.set_character_size(30);	// font size
	menu_title.set_color(&Color::black());	// color
	menu_title.set_string("Welcome to Tic-Tac-Toe");	// what sais
	let menu_title_size = menu_title.get_local_bounds();	// size of text
	let menu_half = menu_title_size.width/2.0;	// find half of text width
	menu_title.set_origin2f(menu_half, 0.0);	// set origin on half of text width
	menu_title.set_position2f(half_width, (menu_y/3.0) - 30.0);	// set postion half of width, third of height


	menu_option.set_font(option_Rc);
	menu_option.set_character_size(20);
	menu_option.set_color(&Color::black());
	menu_option.set_string("Press Space to start");
	let option_size = menu_option.get_local_bounds();
	let option_half = option_size.width/2.0;
	menu_option.set_origin2f(option_half, 0.0);
	menu_option.set_position2f(half_width, (menu_y/2.0) - 20.0);

	(menu_window, menu_title, menu_option)
}