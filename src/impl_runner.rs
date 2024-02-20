use std::io::Write;

use crate::prelude::*;

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
        let mut sz = (0,0);
        let mut i = 0_u16;
        let mut enter_cli = false;
        loop {
            let title = format!(" --> {}", i);
            ConsoleWindow::set_title(&title)?;
            match process_input(&mut enter_cli, &mut sz )? {
                None => {},
                Some( pos ) => {
                    pointer = pos;
                },
            }
            if enter_cli {
                enter_cli = false;
                cw.restore_main_screen()?;
                let mut automouse = false;
                match get_cli_command()?.as_str() {
                    "" => {
                        cw.error( "\n" );
                    },
                    "stop" => {
                        return Err(anyhow!( "exit by command <stop>" ));
                    },
                    "automouse" => {
                        automouse = true;
                    },
                    another => {
                        cw.error( format!("unknown command <{}>\n..IGNORED!", another).as_str() );
                    },
                }
                std::thread::sleep(std::time::Duration::from_millis(200));
                cw.enter_alt_screen( automouse )?;
            }
            //
            app.update();
            {
                let mut cd = cw.get_painter()?;
                process_draw( &mut cd, i, &pointer, &sz )?;
            }
            std::thread::sleep(std::time::Duration::from_millis(1)); // TODO: debug only
            //
            if i >= 55555 {
                break;
            }else{
                i += 1;
            }
        }
        //
        // - [ ] check AppExit
        //
        Ok(())
    }
}

fn get_cli_command() -> Result<String> {
    print!("cli > ");
    std::io::stdout().flush()?;
    for line in std::io::stdin().lines() {
        return Ok(line?);
    }
    Ok( "".to_string() )
}




fn process_input(enter_cli: &mut bool, sz: &mut (u16,u16) ) -> Result< Option<(u16,u16)> > {
    let inputs = ConsoleWindow::read_events()?;
    let mut result: Option< (u16,u16) > = None;
    for event in inputs {
        match event {
            xEvent::Event::Key(key) => {
                if key.code == xEvent::KeyCode::Char('`') {
                    *enter_cli = true;
                }
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
            xEvent::Event::Resize(w, h) => {
                *sz = (w,h);
            },
            _ => {
            },
        }
    }
    Ok( result )
}

fn process_draw( cd: &mut ConsoleDraw, i: u16, pointer: &(u16,u16), sz: &(u16,u16) ) -> Result<()> {
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
    cd  .set_colors( xColors{foreground:Some(xColor::Reset),background:Some(xColor::Black)} )?;
    let info = format!( "size: {},{}", cd.width, cd.height);
    cd  .move_to( 10, 10 )?
        .set_colors( xColors{foreground:None,background:None} )?
        .print(&info)?;
    let info2 = format!( "cursor: {},{}", pointer.0, pointer.1);
    cd  .move_to( 10, 11 )?
        .print(&info2)?;
    let sz = format!( "event size: {},{}", sz.0, sz.1);
    cd  .move_to( 20, 15 )?
        .print(&sz)?;
    Ok(())
}
