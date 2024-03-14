use std::{io::{stdout, Stdout, Write}, time::Duration};

use crossterm::{
    cursor::MoveTo, event::{poll, read, Event, KeyCode}, style::Print, terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType}, ExecutableCommand, QueueableCommand
};

struct World {
    player_c:u16,
    player_l:u16
}

fn draw(mut sc:&Stdout,world: &World)-> std::io::Result<()> {
    // clear the screen
    sc.queue(Clear(ClearType::All))?;
    // move the cursor
    sc.queue(MoveTo(world.player_c,world.player_l))?;
    // draw the pointer(Player)
    sc.queue(Print("P"))?;

    sc.flush();

    Ok(())
}
fn main() -> std::io::Result<()> {
    // 1. init the screen
    let mut sc = stdout();
    // get max row and line user shell
    let(max_column,max_line)= size().unwrap();

    enable_raw_mode()?;
    //2. init the game
    let mut world = World {
        player_c: max_column / 2,
        player_l:max_line - 1
    };

    loop {
 
        // It's guaranteed that the `read()` won't block when the `poll()`
        if poll(Duration::from_millis(10))? {
            let key = read().unwrap();
            match key {
                // Event::FocusGained => println!("FocusGained"),
                // Event::FocusLost => println!("FocusLost"),
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Char('q')=> {break;}
                        KeyCode::Char('w')=> {
                            if world.player_l > 1 { world.player_l -= 1;}
                        }
                        KeyCode::Char('s')=> {
                            if world.player_l < max_line { world.player_l += 1; }
                        }
                        KeyCode::Char('a')=> {
                            if world.player_c > 1 { world.player_c -= 1; }
                        }
                        KeyCode::Char('d')=> {
                            if world.player_c < max_column - 1 { world.player_c += 1; }
                        }
                        _ => {}
                        
                    }
                }
                _ => {}
            }
        } else {
            // Timeout expired and no `Event` is available
        }  
        // physics
        draw(&sc, &world)?;
  
    }

    disable_raw_mode();

    Ok(())
}