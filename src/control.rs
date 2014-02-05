extern mod rsfml;
use rsfml::window::{event, keyboard};
use rsfml::window::keyboard::Key;
use rsfml::graphics::{RenderWindow};

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

pub fn game(which:int, mut is_color:[bool, ..2], turn:bool) -> [bool, ..2] {
	let mut blue_pressed:Key; let mut yellow_pressed:Key;
	let mut block:int;
	// assign button pressed and what block talking about on which
	match which {
		0	=>	{blue_pressed = keyboard::Numpad7; yellow_pressed = keyboard::Numpad7;block = 0;}
		1	=>	{blue_pressed = keyboard::Numpad8; yellow_pressed = keyboard::Numpad8;block = 1;}
		2	=>	{blue_pressed = keyboard::Numpad9; yellow_pressed = keyboard::Numpad9;block = 2;}
		3	=>	{blue_pressed = keyboard::Numpad4; yellow_pressed = keyboard::Numpad4;block = 3;}
		4	=>	{blue_pressed = keyboard::Numpad5; yellow_pressed = keyboard::Numpad5;block = 4;}
		5	=>	{blue_pressed = keyboard::Numpad6; yellow_pressed = keyboard::Numpad6;block = 5;}
		6	=>	{blue_pressed = keyboard::Numpad1; yellow_pressed = keyboard::Numpad1;block = 6;}
		7	=>	{blue_pressed = keyboard::Numpad2; yellow_pressed = keyboard::Numpad2;block = 7;}
		8	=>	{blue_pressed = keyboard::Numpad3; yellow_pressed = keyboard::Numpad3;block = 8;}
		_	=>	{fail!(~"Error, creating button_pressed.");}
	}


	// Make blue
	if keyboard::is_key_pressed(blue_pressed) && which == block && is_color[0] == false && is_color[1] == false && turn == false{is_color = [true, false];} 
	// make yellow
	if keyboard::is_key_pressed(yellow_pressed) && which == block && is_color[1] == false && is_color[0] == false && turn == true{is_color = [false, true];} 

	//otherwise do nothing
	is_color
}

pub fn over(mut is_over:bool) ->  bool {
	if keyboard::is_key_pressed(keyboard::Space) {is_over = false};
	is_over
}