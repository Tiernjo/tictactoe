extern mod rsfml;
use rsfml::graphics::{Color, RectangleShape};

pub fn check(widgets:&~[RectangleShape], turn_count:int) -> (bool, &str) {
	let mut is_over:bool = false;
	let mut winner:&str = "";
	let blue = Color::blue();
	let yellow = Color::yellow();

	if (widgets[0].get_fill_color() == blue) && (widgets[1].get_fill_color() == blue) && (widgets[2].get_fill_color() == blue) {is_over = true; winner = "blue";}	// Top row blue
	if (widgets[3].get_fill_color() == blue) && (widgets[4].get_fill_color() == blue) && (widgets[5].get_fill_color() == blue) {is_over = true;winner = "blue";}	// Mid row blue
	if (widgets[6].get_fill_color() == blue) && (widgets[7].get_fill_color() == blue) && (widgets[8].get_fill_color() == blue) {is_over = true;winner = "blue";}	// Bot row blue
	if (widgets[0].get_fill_color() == blue) && (widgets[3].get_fill_color() == blue) && (widgets[6].get_fill_color() == blue) {is_over = true;winner = "blue";}	// Top column blue
	if (widgets[1].get_fill_color() == blue) && (widgets[4].get_fill_color() == blue) && (widgets[7].get_fill_color() == blue) {is_over = true;winner = "blue";}	// Mid column blue
	if (widgets[2].get_fill_color() == blue) && (widgets[5].get_fill_color() == blue) && (widgets[8].get_fill_color() == blue) {is_over = true;winner = "blue";}	// Bot column blue
	if (widgets[0].get_fill_color() == blue) && (widgets[4].get_fill_color() == blue) && (widgets[8].get_fill_color() == blue) {is_over = true;winner = "blue";}	// \ blue
	if (widgets[6].get_fill_color() == blue) && (widgets[4].get_fill_color() == blue) && (widgets[2].get_fill_color() == blue) {is_over = true;winner = "blue";}	// / blue

	if (widgets[0].get_fill_color() == yellow) && (widgets[1].get_fill_color() == yellow) && (widgets[2].get_fill_color() == yellow) {is_over = true;winner = "yellow";}	// Top row yellow
	if (widgets[3].get_fill_color() == yellow) && (widgets[4].get_fill_color() == yellow) && (widgets[5].get_fill_color() == yellow) {is_over = true;winner = "yellow";}	// Mid row yellow
	if (widgets[6].get_fill_color() == yellow) && (widgets[7].get_fill_color() == yellow) && (widgets[8].get_fill_color() == yellow) {is_over = true;winner = "yellow";}	// Bot row yellow
	if (widgets[0].get_fill_color() == yellow) && (widgets[3].get_fill_color() == yellow) && (widgets[6].get_fill_color() == yellow) {is_over = true;winner = "yellow";}	// Top column yellow
	if (widgets[1].get_fill_color() == yellow) && (widgets[4].get_fill_color() == yellow) && (widgets[7].get_fill_color() == yellow) {is_over = true;winner = "yellow";}	// Mid column yellow
	if (widgets[2].get_fill_color() == yellow) && (widgets[5].get_fill_color() == yellow) && (widgets[8].get_fill_color() == yellow) {is_over = true;winner = "yellow";}	// Bot column yellow
	if (widgets[0].get_fill_color() == yellow) && (widgets[4].get_fill_color() == yellow) && (widgets[8].get_fill_color() == yellow) {is_over = true;winner = "yellow";}	// \ yellow
	if (widgets[6].get_fill_color() == yellow) && (widgets[4].get_fill_color() == yellow) && (widgets[2].get_fill_color() == yellow) {is_over = true;winner = "yellow";}	// / yellow
	
	if turn_count == 9 {is_over = true; winner = "cat";}
	(is_over, winner)
}