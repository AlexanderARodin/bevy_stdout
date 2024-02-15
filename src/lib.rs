pub mod prelude;
use bevy::prelude::*;

mod impl_stdout_plugin;
pub struct StdoutPlugin;
impl Plugin for StdoutPlugin {
    fn build(&self, app: &mut App) {
        app.set_runner(Self::runner_wrapper);
    }
}



// TODO: debug only
#[derive(Resource)]
pub struct UpdateIndex(pub u32);

