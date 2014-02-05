extern mod rsfml;
use rsfml::graphics::{Color, RectangleShape};

pub fn check(widgets:&~[RectangleShape]) -> (bool, bool) {
	let mut is_over:bool = false;
	let mut winner:bool = false;
	let blue = Color::blue();
	let yellow = Color::yellow();

	if (widgets[0].get_fill_color() == blue) && (widgets[1].get_fill_color() == blue) && (widgets[2].get_fill_color() == blue) {is_over = true; winner = true;}	// Top row blue
	if (widgets[3].get_fill_color() == blue) && (widgets[4].get_fill_color() == blue) && (widgets[5].get_fill_color() == blue) {is_over = true;winner = true;}	// Mid row blue
	if (widgets[6].get_fill_color() == blue) && (widgets[7].get_fill_color() == blue) && (widgets[8].get_fill_color() == blue) {is_over = true;winner = true;}	// Bot row blue
	if (widgets[0].get_fill_color() == blue) && (widgets[3].get_fill_color() == blue) && (widgets[6].get_fill_color() == blue) {is_over = true;winner = true;}	// Top column blue
	if (widgets[1].get_fill_color() == blue) && (widgets[4].get_fill_color() == blue) && (widgets[7].get_fill_color() == blue) {is_over = true;winner = true;}	// Mid column blue
	if (widgets[2].get_fill_color() == blue) && (widgets[5].get_fill_color() == blue) && (widgets[8].get_fill_color() == blue) {is_over = true;winner = true;}	// Bot column blue
	if (widgets[0].get_fill_color() == blue) && (widgets[4].get_fill_color() == blue) && (widgets[8].get_fill_color() == blue) {is_over = true;winner = true;}	// \ blue
	if (widgets[6].get_fill_color() == blue) && (widgets[4].get_fill_color() == blue) && (widgets[2].get_fill_color() == blue) {is_over = true;winner = true;}	// / blue

	if (widgets[0].get_fill_color() == yellow) && (widgets[1].get_fill_color() == yellow) && (widgets[2].get_fill_color() == yellow) {is_over = true;winner = false;}	// Top row yellow
	if (widgets[3].get_fill_color() == yellow) && (widgets[4].get_fill_color() == yellow) && (widgets[5].get_fill_color() == yellow) {is_over = true;winner = false;}	// Mid row yellow
	if (widgets[6].get_fill_color() == yellow) && (widgets[7].get_fill_color() == yellow) && (widgets[8].get_fill_color() == yellow) {is_over = true;winner = false;}	// Bot row yellow
	if (widgets[0].get_fill_color() == yellow) && (widgets[3].get_fill_color() == yellow) && (widgets[6].get_fill_color() == yellow) {is_over = true;winner = false;}	// Top column yellow
	if (widgets[1].get_fill_color() == yellow) && (widgets[4].get_fill_color() == yellow) && (widgets[7].get_fill_color() == yellow) {is_over = true;winner = false;}	// Mid column yellow
	if (widgets[2].get_fill_color() == yellow) && (widgets[5].get_fill_color() == yellow) && (widgets[8].get_fill_color() == yellow) {is_over = true;winner = false;}	// Bot column yellow
	if (widgets[0].get_fill_color() == yellow) && (widgets[4].get_fill_color() == yellow) && (widgets[8].get_fill_color() == yellow) {is_over = true;winner = false;}	// \ yellow
	if (widgets[6].get_fill_color() == yellow) && (widgets[4].get_fill_color() == yellow) && (widgets[2].get_fill_color() == yellow) {is_over = true;winner = false;}	// / yellow


	(is_over, winner)
}