extern crate gfx_texture;
extern crate image;
extern crate piston;
extern crate piston_window;
extern crate rand;
extern crate texture;

use gfx_texture::Texture;
use image::ImageBuffer;
use piston::input::*;
use piston_window::*;
use rand::Rng;
use std::time;
use texture::TextureSettings;

const WIDTH: u32 = 60;  // Width of window, in cells.
const HEIGHT: u32 = 30;  // Height of window, in cells.
const CELL_SIZE: u32 = 20;  // Effectively a zoom factor.  CELL_SIZE * WIDTH => width of the window in pixels.
const TICK_PERIOD: u32 = 1500;  // The number of milliseconds between each recalculation of the cell grid.

fn main() {
	let mut window : PistonWindow = WindowSettings::new("conway", [WIDTH * CELL_SIZE, HEIGHT * CELL_SIZE])
		.exit_on_esc(true)
		.build()
		.expect("error making window");

	// Since we're modeling one cell per pixel, the texture gets magnified.  Setting the
	// mag filter to Nearest gives us sharp edges on the cells.
	let tset = TextureSettings::new()
		.mag(texture::Filter::Nearest);

	// Initialize an array of pixels which can be arbitrarily white or black.
	let mut random_pixels = [0u8; (WIDTH * HEIGHT * 4) as usize];
	let mut rng = rand::thread_rng();
	for pixel in random_pixels.chunks_mut(4) {
		if (rng.next_u32() % 4) != 0 {
			pixel[0] = 255;
			pixel[1] = 255;
			pixel[2] = 255;
		}
		pixel[3] = 255;
	}

	// This is our cell buffer.  When using the iterators provided by ImageBuffer, p.0 and p.1
	// are the x, y coordinates, and p.2 is the four-byte array that represents RGBA.
	let mut cells = ImageBuffer::from_raw(WIDTH, HEIGHT, random_pixels.to_vec()).unwrap();
	let mut tex = Texture::from_image(&mut window.factory, &cells, &tset).unwrap();

	// Time keeping to know when to recalculate the cell grid.
	let tick_period = time::Duration::from_millis(TICK_PERIOD as u64);
	let mut last_tick = time::Instant::now();

	// Don't start the game until the user presses a key.  This gives the "player" an opportunity
	// to observe the starting state.
	while let Some(e) = window.next() {
		window.draw_2d(&e, |c, g| {
			clear([0.0, 0.0, 0.0, 1.0], g);
			image(&tex, c.transform.scale(CELL_SIZE as f64, CELL_SIZE as f64), g);
		});

		if e.press_args().is_some() {
			break;
		}
	}

	// Main game loop: continue recalculating and rendering once per tick forever.
	while let Some(e) = window.next() {
		// Recalculate the cell field once per tick.
		if last_tick.elapsed() > tick_period {
			for p in cells.enumerate_pixels_mut() {
				p.2.data = [0, 0, 0, 255];
			}
			tex = Texture::from_image(&mut window.factory, &cells, &tset).unwrap();
			last_tick = time::Instant::now();
		}

		// This is the draw step.  In our case it's very simple - clear the window then render
		// the image buffer as a scaled-up texture.
		window.draw_2d(&e, |c, g| {
			clear([0.0, 0.0, 0.0, 1.0], g);
			image(&tex, c.transform.scale(CELL_SIZE as f64, CELL_SIZE as f64), g);
		});
	}
}
