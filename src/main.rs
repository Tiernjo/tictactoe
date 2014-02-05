extern mod native;
extern mod rsfml;
extern mod std;
mod control;
mod grid;
mod menu;
mod run;
mod widget;
mod window;

// Macs need rsfml to start on main thread
#[cfg(target_os="macos")]
#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

fn main() {
	//run program
	::run::main_loop();
}
