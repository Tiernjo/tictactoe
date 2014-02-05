extern mod rsfml;
use rsfml::graphics::{Color, RectangleShape, RenderWindow};
use rsfml::graphics::rc::Text;

mod control;
mod grid;
mod menu;
mod window;
mod widget;


// Create a main loop
pub fn main_loop() {
	let mut is_playing = false;

	// Get Window
	let mut window = ::window::create();

	// Create Main menu
	let (menu, menu_title, menu_option) = ::menu::create(window.get_size());
	//menu_text.set_font(&menu_font);

	// Get Lines for grid and border
	let (left_line, right_line, top_line, bottom_line) = ::grid::create("grid");
	let (left_border, right_border, top_border, bottom_border) = ::grid::create("border");
	
	// Get Widgets
	let mut widgets = ::widget::create();
	// Take out the elemnt 0 and make variable with it
	// expect() is an error system, like the match Some()/None() but shorter
	let mut top_left = widgets.remove(0).expect("tl"); let mut top_mid = widgets.remove(0).expect("tm"); let top_right = widgets.remove(0).expect("tr");
	let mid_left = widgets.remove(0).expect("ml"); let mid_mid = widgets.remove(0).expect("mm"); let mid_right = widgets.remove(0).expect("mr");
	let bot_left = widgets.remove(0).expect("bl"); let bot_mid = widgets.remove(0).expect("bm"); let bot_right = widgets.remove(0).expect("br");

	top_left.set_fill_color(&Color::yellow());

	while window.is_open() {
		::control::input(&mut window);

		if is_playing == true {
			show_game(&mut window, 
				&left_line, &right_line, &top_line, &bottom_line,
				&left_border, &right_border, &top_border, &bottom_border,
				&top_left, &top_mid, &top_right,
				&mid_left, &mid_mid, &mid_right,
				&bot_left, &bot_mid, &bot_right);
		} else {
			is_playing = ::control::menu(is_playing);
			show_menu(&mut window,
				&menu, &menu_title, &menu_option);
		}
	}
}

fn show_menu(window: &mut RenderWindow,
	menu: &RectangleShape, menu_text: &Text, menu_option: &Text) {
	
	window.clear(&Color::red());
	window.draw(menu); window.draw(menu_text); window.draw(menu_option);
	window.display()
}

// Clear buffer, draw items, show results
fn show_game(window: &mut RenderWindow,
	left_line: &RectangleShape, right_line: &RectangleShape, 
	top_line: &RectangleShape, bottom_line: &RectangleShape,
	left_border: &RectangleShape, right_border: &RectangleShape,
	top_border: &RectangleShape, bottom_border: &RectangleShape,
	top_left: &RectangleShape, top_mid: &RectangleShape, top_right: &RectangleShape,
	mid_left: &RectangleShape, mid_mid: &RectangleShape, mid_right: &RectangleShape,
	bot_left: &RectangleShape, bot_mid: &RectangleShape, bot_right: &RectangleShape) {

	// Clean out window
	window.clear(&Color::red());

	// Draw grid_lines
	window.draw(left_line); window.draw(right_line); 
	window.draw(top_line); window.draw(bottom_line);
	window.draw(left_border); window.draw(right_border);
	window.draw(top_border); window.draw(bottom_border);

	// Draw widgets
	window.draw(top_left); window.draw(top_mid); window.draw(top_right);
	window.draw(mid_left); window.draw(mid_mid); window.draw(mid_right);
	window.draw(bot_left); window.draw(bot_mid); window.draw(bot_right);

	// Display window
	window.display()
}