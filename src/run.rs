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
	let mut is_blue = false;
	let mut i:int = 0;

	// Get Window
	let mut window = ::window::create();

	// Create Main menu
	let (menu, menu_title, menu_option) = ::menu::create(window.get_size());
	//menu_text.set_font(&menu_font);

	// Get Lines for grid and border
	let (left_line, right_line, top_line, bottom_line) = ::grid::create("grid");
	let (left_border, right_border, top_border, bottom_border) = ::grid::create("border");
	
	// Get Widgets
	let (mut widgets, widget_bounds) = ::widget::create();
	

	while window.is_open() {
		::control::exit(&mut window);

		if is_playing == true {	
			while i < 9 {
				let is_blue = ::control::game(i, is_blue);
				if is_blue == true {widgets[0].set_fill_color(&Color::blue());}
				i += 1;
			}
			i = 0;			
			
			show_game(&mut window, 
				&left_line, &right_line, &top_line, &bottom_line,
				&left_border, &right_border, &top_border, &bottom_border,
				&widgets);
		} else {
			// Show Title Menu
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
	widgets: &~[RectangleShape]){

	// Clean out window
	window.clear(&Color::red());

	// Draw grid_lines
	window.draw(left_line); window.draw(right_line); 
	window.draw(top_line); window.draw(bottom_line);
	window.draw(left_border); window.draw(right_border);
	window.draw(top_border); window.draw(bottom_border);

	// Draw widgets
	window.draw(&widgets[0]);window.draw(&widgets[1]);window.draw(&widgets[2]);
	window.draw(&widgets[3]);window.draw(&widgets[4]);window.draw(&widgets[5]);
	window.draw(&widgets[6]);window.draw(&widgets[7]);window.draw(&widgets[8]);

	// Display window
	window.display()
}