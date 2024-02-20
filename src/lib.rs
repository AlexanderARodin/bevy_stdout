pub mod prelude;
    use prelude::*;
use bevy::prelude::*;

mod impl_runner;

pub struct StdoutPlugin;
impl Plugin for StdoutPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource( UpdateIndex( 111 ) );
        app.set_runner(Self::runner_wrapper);
    }
}
impl Drop for StdoutPlugin {
    fn drop(&mut self) {
        //todo!("DROPPER");
        println!("\nStdoutPlugin: DROPPER\n");
    }
}

