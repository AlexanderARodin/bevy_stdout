//use crate::prelude::*;

use anyhow::anyhow;
use bevy::prelude::*;
use console_window::prelude::*;


use crate::StdoutPlugin;
impl StdoutPlugin {

    pub(crate) fn runner_wrapper( mut app: App ) {
        println!("  ---> loop_runner.."); // TODO: debug only
        //
        match Self::internal_runner( &mut app ) {
            Err(e) => {
                eprintln!("E: {}", e.to_string());
            },
            Ok( () ) => {},
        };
        //
        println!("  <--- ..loop_runner"); // TODO: debug only
    }
    fn internal_runner( app: &mut App ) -> Result<()> {
        let mut cw = ConsoleWindow::new()?; // for auto-restoring after Drop
        cw.info("enter looping..");
        cw.enter_alt_screen(true)?;
        //
        // loop
        let mut pointer = (0,0);
        for i in 0..=65535 {
            let title = format!(" --> {}", i);
            ConsoleWindow::set_title(&title)?;
            match process_input()? {
                None => {},
                Some( pos ) => {
                    pointer = pos;
                },
            }
            app.update();
            {
                let mut cd = cw.get_painter()?;
                process_draw( &mut cd, i, &pointer )?;
            }
            std::thread::sleep(std::time::Duration::from_millis(1)); // TODO: debug only
        }
        //
        // - [ ] check AppExit
        //
        Ok(())
    }
}





fn process_input() -> Result< Option<(u16,u16)> > {
    let inputs = ConsoleWindow::read_events()?;
    let mut result: Option< (u16,u16) > = None;
    for event in inputs {
        match event {
            xEvent::Event::Key(key) => {
                if key.code == xEvent::KeyCode::Char('c') {
                    if key.modifiers .contains( xEvent::KeyModifiers::CONTROL ) {
                        return Err(anyhow!( "<C-c>" ));
                    }
                }else if key.code == xEvent::KeyCode::Esc {
                    return Err(anyhow!( "Esc" ));
                }
            },
            xEvent::Event::Mouse( mouse_event ) => {
               result = Some( (mouse_event.column, mouse_event.row) );
            },
            _ => {
            },
        }
    }
    Ok( result )
}

fn process_draw( cd: &mut ConsoleDraw, i: u16, pointer: &(u16,u16) ) -> Result<()> {
    cd  .move_to( i/100, i/100 )?
        .print( "x---------------------------------------------------------x" )?;
    //
    cd  .set_colors( xColors{foreground:Some(xColor::Black),background:Some(xColor::Blue)} )?;
    //
    cd  .move_to( pointer.0, pointer.1 )?
        .print("+")?;
    if pointer.0 >= 5 {
        cd  .move_to( pointer.0 - 5, pointer.1 )?
            .print(">")?;
    }
    if (pointer.0+5) < cd.width {
        cd  .move_to( pointer.0 + 5, pointer.1 )?
            .print("<")?;
    }
    if (pointer.1+1) < cd.height {
        cd  .move_to( pointer.0, pointer.1+1 )?
            .print("^")?;
    }
    //
    cd  .set_colors( xColors{foreground:Some(xColor::Reset),background:Some(xColor::Grey)} )?;
    let info = format!( "size: {},{}", cd.width, cd.height);
    cd  .move_to( 10, 10 )?
        .set_colors( xColors{foreground:None,background:None} )?
        .print(&info)?;
    let info2 = format!( "cursor: {},{}", pointer.0, pointer.1);
    cd  .move_to( 10, 11 )?
        .print(&info2)?;
    Ok(())
}
