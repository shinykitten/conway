extern crate gfx_texture;
extern crate image;
extern crate piston;
extern crate piston_window;
extern crate texture;

use gfx_texture::Texture;
use image::ImageBuffer;
use piston::input::*;
use piston_window::*;
use texture::TextureSettings;

const WIDTH: u32 = 60;  // Width of window, in cells.
const HEIGHT: u32 = 60;  // Height of window, in cells.
const CELL_SIZE: u32 = 10;  // Effectively a zoom factor.  CELL_SIZE * WIDTH => width of the window in pixels.

fn main() {
	let mut window : PistonWindow = WindowSettings::new("conway", [WIDTH * CELL_SIZE, HEIGHT * CELL_SIZE])
		.exit_on_esc(true)
		.build()
		.expect("error making window");

	// Since we're modeling one cell per pixel, the texture gets magnified.  Setting the
	// mag filter to Nearest gives us sharp edges on the cells.
	let tset = TextureSettings::new()
		.mag(texture::Filter::Nearest);

	let buf = vec![0u8; (WIDTH * HEIGHT * 4) as usize];
	let mut cells = ImageBuffer::from_raw(WIDTH, HEIGHT, buf).unwrap();
	let mut tex = Texture::from_image(&mut window.factory, &cells, &tset).unwrap();

	while let Some(e) = window.next() {
		if let Some(Button::Keyboard(key)) = e.press_args() {
			if key == Key::Space {
				for p in cells.enumerate_pixels_mut() {
					if p.0 % 2 == p.1 % 2 {
						p.2.data = [0xFF, 0xFF, 0xFF, 0xFF];
					}
				}
				tex = Texture::from_image(&mut window.factory, &cells, &tset).unwrap();
			}
		}
		window.draw_2d(&e, |c, g| {
			clear([0.0, 0.0, 0.0, 1.0], g);
			image(&tex, c.transform.scale(CELL_SIZE as f64, CELL_SIZE as f64), g);
		});
	}
}
