use crate::prelude::*;
use bevy::prelude::*;

use std::io::{Stdout,Write};
use crossterm::{QueueableCommand,ExecutableCommand};
use crossterm::terminal as xxxterm;
use crossterm::cursor   as xxxcursor;
use crossterm::event    as xxxevent;
use crossterm::event::Event    as xEvent;

static POLL_WAIT_TIME: std::time::Duration = std::time::Duration::from_secs(0);

use crate::StdoutPlugin;
impl StdoutPlugin {

    pub(crate) fn runner_wrapper( mut app: App ) {
        println!("  ---> loop_runner.."); // TODO: debug only
        //
        let mut stdout = std::io::stdout();
        match prepare_stdout( &mut stdout ) {
            Ok(()) => {
                match Self::stdout_loop( &mut app, &mut stdout ) {
                    Err(e) => {
                        let _ = stdout
                                .execute( xxxterm::LeaveAlternateScreen );
                        eprintln!( "\nE: <{}>", e.to_string() );
                        eprintln!( "E: terminate <stdout_loop>");
                    },
                    _ => {
                    },
                }
            },
            Err(e) => {
                let _ = stdout
                        .execute( xxxterm::LeaveAlternateScreen );
                eprintln!( "\nE: <{}>", e.to_string() );
                eprintln!( "E: unable to invoke <prepare_stdout>");
            },
        }
        match restore_stdout( &mut stdout ) {
            Err(e) => {
                eprintln!( "\nE: <{}>", e.to_string() );
                eprintln!( "E: unable to invoke <restore_stdout>");
            },
            _ => {},
        }
        println!("  <--- ..loop_runner"); // TODO: debug only
    }

    fn stdout_loop( app: &mut App, stdout: &mut Stdout ) -> ResultOf< () > {
        app.insert_resource( UpdateIndex( 111 ) );

        loop {
            match app.world.get_resource_mut::<UpdateIndex>() {
                Some( mut i ) if i.0 > 0 => {
                    i.0 -= 1;
                },
                Some( _ ) => {
                    println!( "           x----- UpdateIndex is Zero" );
                    break;
                },
                None => {
                    return Err(Box::from( "UpdateIndex is None" ));
                },
            }
            Self::process_inputs()?;

            // - [ ] redraw terminal 
            
            app.update();

            std::thread::sleep(std::time::Duration::from_millis(150)); // TODO: debug only
            stdout
                .execute(
                    xxxcursor::MoveTo( 0, 20 )
                    );

            // - [ ] check AppExit
            //
        }

        Ok(())
    }

    fn process_inputs() -> ResultOf< () > {
        loop {
            match xxxevent::poll( POLL_WAIT_TIME ) {
                Ok(true) => {
                    match xxxevent::read()? {
                        xEvent::Key( key ) => {
                            print!("key happens");
                           if key.code == xxxevent::KeyCode::Char('c') {
                                if key.modifiers.contains(xxxevent::KeyModifiers::CONTROL) {
                                    return Err(Box::from( "<C-c>" ));
                                }
                            } else if key.code == xxxevent::KeyCode::Esc {
                                    return Err(Box::from( "<Esc>" ));
                            }
                        },
                        xEvent::Mouse( _mouse ) => {
                            print!("mouse happens");
                        },
                        xEvent::Resize( new_width, new_height ) => {
                            print!("resize ({},{})", new_width, new_height);
                        },
                        _ => {
                            print!("other events");
                        },
                    }
                },
                _ => {
                    return Ok(());
                },
            }
        }
    }

}

fn prepare_stdout( stdout: &mut Stdout ) -> ResultOf< () > {
    stdout
        .queue(xxxterm::EnterAlternateScreen)?
        .queue(xxxevent::EnableMouseCapture)?
        .queue(xxxterm::Clear( xxxterm::ClearType::All ) )?;
    xxxterm::enable_raw_mode()?;
    stdout.flush()?;
    Ok(())
}
fn restore_stdout( stdout: &mut Stdout ) -> ResultOf< () > {
    xxxterm::disable_raw_mode()?;
    stdout
        .execute(xxxevent::DisableMouseCapture)?
        .execute(xxxterm::LeaveAlternateScreen)?
        .execute(xxxcursor::Show)?;
    Ok(())
}






// TODO: debug only
#[derive(Resource)]
pub struct UpdateIndex(pub u32);

