extern mod rsfml;
use rsfml::graphics::{Color, RectangleShape, RenderWindow};
use rsfml::graphics::rc::Text;

mod control;
mod grid;
mod menu;
mod over;
mod window;
mod widget;


// Create a main loop
pub fn main_loop() {
	let (mut is_playing, mut is_over) = (false, false);	// Title to game, game to who one
	let (is_blue, is_yellow) = (false, false);	// block is blue, block is yellow
	let mut is_color = [is_blue, is_yellow];	// vector of if blue, if yellow
	let mut i:int = 0;							// iterator variable
	let mut winner:bool = false;

	// Get Window
	let mut window = ::window::create();

	// Create Main menu
	let (menu, menu_title, menu_option) = ::menu::create(window.get_size(), "Welcome to Tic-Tac-Toe", "Press space to play");
	// Create End menu
	let (blue_end, blue_title, blue_option) = ::menu::create(window.get_size(), "Blue wins", "Press esc to end");
	let (yellow_end, yellow_title, yellow_option) = ::menu::create(window.get_size(), "Yellow wins", "Press esc to end");

	// Get Lines for grid and border
	let (left_line, right_line, top_line, bottom_line) = ::grid::create("grid");
	let (left_border, right_border, top_border, bottom_border) = ::grid::create("border");
	
	// Get Widgets
	let (mut widgets, widget_bounds) = ::widget::create();	

	while window.is_open() {
		::control::exit(&mut window);

		// Main Menu
		if is_playing == false{
			// Show Title Menu
			is_playing = ::control::menu(is_playing);
			show_menu(&mut window, &menu, &menu_title, &menu_option);
		// Game Screen
		} else if is_playing == true && is_over == false{	
			// Run through widgets
			while i < 9 {
				is_color = ::control::game(i, is_color);
				let current_fill_color = widgets[i].get_fill_color();
				if is_color[0] == true && is_color[1] == false && current_fill_color == Color::white() {widgets[i].set_fill_color(&Color::blue());}	
					else if is_color[1] == true && is_color[0] == false && current_fill_color == Color::white(){widgets[i].set_fill_color(&Color::yellow());}		
				is_color = [is_blue, is_yellow];	// reset is_color to original values
				i += 1;
			}
			i = 0;
			let (pulled_over, pulled_winner) = ::over::check(&widgets);
			is_over = pulled_over; winner = pulled_winner;

			show_game(&mut window, 
				&left_line, &right_line, &top_line, &bottom_line,
				&left_border, &right_border, &top_border, &bottom_border,
				&widgets);
			
		// End Game
		} else if is_over == true {
			if winner == true {
				show_menu(&mut window, &blue_end, &blue_title, &blue_option);
			} else if winner == false {
				show_menu(&mut window, &yellow_end, &yellow_title, &yellow_option);
			}
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