extern crate piston_window;
extern crate rand;

use piston_window::{EventLoop, PistonWindow, WindowSettings};
use piston_window::{PressEvent, ReleaseEvent, RenderEvent, UpdateEvent};

mod settings;
mod resources;
mod view;
mod models;
mod controller;
mod utils;

use resources::Resources;
use models::{Scene, World};
use controller::{Controller, Timer};

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Tetris", [16 * 20, 16 * 24])
        .exit_on_esc(true)
        .build()
        .expect("Could not create window");
    window.set_ups(60);

    let mut rng = rand::thread_rng();

    let mut resources = Resources::new(&mut window);
    let mut world = World::new(&mut rng);
    let mut controller = Controller::new();
    let mut timer = Timer::new();
    let mut scene = Scene::Playing;

    while let Some(e) = window.next() {
        if let Some(ref button) = e.press_args() {
            controller.handle_button(button, true);
        }
        if let Some(ref button) = e.release_args() {
            controller.handle_button(button, false);
        }
        if let Some(args) = e.update_args() {
            if scene == Scene::Playing {
                timer.update_current_time(args.dt);
            }
            controller.update_game(&mut world, &mut rng, &mut timer, &mut scene);
        }
        if let Some(_) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                view::render_game(&c, g, &world, &mut resources, &scene);
            });
        }
    }
}
