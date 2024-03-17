use sdl2::event::Event;

use crate::{
    common::resources::{input::Input, window::Window},
    prelude::World,
    schedule::Update,
    QPResult,
};

use super::Plugin;

pub struct InputPlugin {}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut crate::prelude::App) -> QPResult<()> {
        app.add_resource(Input::new());

        app.add_system(Update, move |world: &mut World| {
            let Some(window) = world.resources.get::<Window>() else {
                #[cfg(debug_assertions)]
                println!("[input system] couldn't get window resource");

                return;
            };

            let events = window.winapi.get_event_queue().unwrap();
            let Some(input) = world.resources.get_mut::<Input>() else {
                #[cfg(debug_assertions)]
                println!("[input system] couldn't get input resource");

                return;
            };

            let mut quit = false;
            for event in events.iter() {
                match event {
                    Event::Quit { .. } => quit = true,
                    event => input.update_state(event),
                }
            }

            if quit {
                world.quit();
            }
        });

        Ok(())
    }
}
