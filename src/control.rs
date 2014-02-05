extern mod rsfml;
use rsfml::window::{event, keyboard};
use rsfml::graphics::{RenderWindow};

pub fn input(window: &mut RenderWindow) {
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