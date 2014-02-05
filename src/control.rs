extern mod rsfml;
use rsfml::window::{event, keyboard};
use rsfml::window::mouse::MouseButton;
use rsfml::graphics::{Color, FloatRect, RectangleShape, RenderWindow};

pub fn exit(window: &mut RenderWindow) {
	loop {
		match window.poll_event() {
			event::Closed => window.close(),
			event::NoEvent => break,
			_ => {}
		}
	}
}

pub fn menu(mut is_playing:bool) -> bool {
	if keyboard::is_key_pressed(keyboard::Space) {is_playing = true};

	match is_playing {
		true	=>	return true,
		false	=>	return false,
	}
}

pub fn game(which:int, is_blue:bool) -> bool {
	if keyboard::is_key_pressed(keyboard::S) && which == 0{
		return !is_blue
	}
	is_blue
}