use bevy::prelude::*;

use bevy_stdout::prelude::*;


fn main() {
    println!("\n--> INTRO");

    App::new()
        .add_plugins(StdoutPlugin)
//        .add_systems(Update, test_system.run_if(resource_exists::<UpdateIndex>()) )
        .run();

    println!("<-- OUTRO\n");
}


//
//fn test_system( update_index: Res<UpdateIndex> ) {
//    println!("   <--> test_system: #{}", update_index.0);
//}
