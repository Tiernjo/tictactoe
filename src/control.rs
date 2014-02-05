extern mod rsfml;
use rsfml::window::{event, keyboard};
use rsfml::window::keyboard::Key;
use rsfml::window::mouse::MouseButton;
use rsfml::graphics::{Color, FloatRect, RectangleShape, RenderWindow};

pub fn exit(window: &mut RenderWindow) {
	loop {
		match window.poll_event() {
			event::Closed => window.close(),
			event::NoEvent => break,
			_ => {}
		}
		if keyboard::is_key_pressed(keyboard::Escape) {window.close()}
	}
}

pub fn menu(mut is_playing:bool) -> bool {
	if keyboard::is_key_pressed(keyboard::Space) {is_playing = true};

	match is_playing {
		true	=>	return true,
		false	=>	return false,
	}
}

pub fn game(which:int, is_color_import:[bool, ..2]) -> [bool, ..2] {
	let mut blue_pressed:Key; let mut yellow_pressed:Key;
	let mut is_color = is_color_import;
	let mut block:int;
	// assign button pressed and what block talking about on which
	match which {
		0	=>	{blue_pressed = keyboard::Num1; yellow_pressed = keyboard::Q;block = 0;}
		1	=>	{blue_pressed = keyboard::Num2; yellow_pressed = keyboard::W;block = 1;}
		2	=>	{blue_pressed = keyboard::Num3; yellow_pressed = keyboard::E;block = 2;}
		3	=>	{blue_pressed = keyboard::Num4; yellow_pressed = keyboard::R;block = 3;}
		4	=>	{blue_pressed = keyboard::Num5; yellow_pressed = keyboard::T;block = 4;}
		5	=>	{blue_pressed = keyboard::Num6; yellow_pressed = keyboard::Y;block = 5;}
		6	=>	{blue_pressed = keyboard::Num7; yellow_pressed = keyboard::U;block = 6;}
		7	=>	{blue_pressed = keyboard::Num8; yellow_pressed = keyboard::I;block = 7;}
		8	=>	{blue_pressed = keyboard::Num9; yellow_pressed = keyboard::O;block = 8;}
		_	=>	{fail!(~"Error, creating button_pressed.");}
	}


	// Make blue
	if keyboard::is_key_pressed(blue_pressed) && which == block && is_color[0] == false && is_color[1] == false {is_color = [true, false]} 
		//if keyboard::is_key_pressed(blue_pressed) && which == block && is_blue == true && is_yellow == false{return (!is_blue, is_yellow)}
	// make yellow
	if keyboard::is_key_pressed(yellow_pressed) && which == block && is_color[1] == false && is_color[0] == false {is_color = [false, true]} 
		//if keyboard::is_key_pressed(yellow_pressed) && which == block && is_yellow == true && is_blue == false{return (is_blue, !is_yellow)}

	//otherwise do nothing
	is_color
}