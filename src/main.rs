use console::{Term, Key};

use std::{io, thread, time::Duration};

fn main() -> Result<(), io::Error> {
    let mut term = Term::stdout();

    term.clear_screen()?;

    let (height, width) = term.size();

    let mut entity_positions: Vec<(u16, u16)> = Vec::new();
    entity_positions.push(((width / 2), (height / 2)));
    let mut z = 0;

    let mut bg = vec![' '; width as usize * height as usize];

    let mut update = vec![(0, 0); 20];
    let mut updated_count = 0;

    let mut auto = false;

    loop {
        {//player specific input handling
            let (mut x, mut y) = entity_positions[0];
            let mut z_update = false;
            update[0] = (x, y);
            updated_count += 1;
            if !auto {
                match term.read_key().unwrap() {
                    Key::Escape => break,
                    Key::Char('a') => { if x > 0            { x -= 1 } }
                    Key::Char('d') => { if x < width - 1    { x += 1 } }
                    Key::Char('w') => { if y > 0            { y -= 1 } }
                    Key::Char('s') => { if y < height - 1   { y += 1 } }
                    Key::Char('e') | Key::Char('E') => { entity_positions.push((x, y)) }
                    Key::Del =>  { if z < 10 { z += 1 } }
                    Key::End =>  { if z > 0 { z -= 1 } }
                    Key::Char('A') => { bg[((y * width) + x) as usize] = '<'; z_update = true },
                    Key::Char('D') => { bg[((y * width) + x) as usize] = '>'; z_update = true },
                    Key::Char('W') => { bg[((y * width) + x) as usize] = '^'; z_update = true },
                    Key::Char('S') => { bg[((y * width) + x) as usize] = 'v'; z_update = true },
                    Key::Enter => auto = true,
                    _ => ()
                }
            }
            

            if z_update {
                for i in 1..=z {
                    if y + i < height && bg[(((y + i) * width) + x) as usize] == ' ' {
                        bg[(((y + i) * width) + x) as usize] = 
                            "\\/".as_bytes()[((y + i + (x % 2)) % 2) as usize] as char;
                        update[updated_count + 1] = (x, y + i);
                        updated_count += 1;
                    }
                }
            }

            entity_positions[0] = (x, y);
        }
        //all entity handling
        for i in 0..entity_positions.len() {
            entity_positions[i] = {
                let (mut x, mut y) = entity_positions[i];

                update[updated_count + 1] = (x, y);
                updated_count += 1;

                match bg[((y * width) + x) as usize] {
                    '<' => { if x > 0       { x -= 1 } }
                    '>' => { if x < width   { x += 1 } }
                    '^' => { if y > 0       { y -= 1 } }
                    'v' => { if y < height  { y += 1 } }
                    _ => ()
                }

                x = x.max(0).min(width - 1);
                y = y.max(0).min(height - 1);

                (x, y)
            }
        }

        for i in 0..= updated_count {
            let (lx, ly) = update[i];
            let last = &format!("\x1b[34m{}\x1b[37m", bg[((ly * width) + lx) as usize]);

            term.move_cursor_to(lx as usize, ly as usize)?;
            term.write_line(last)?;

            update[i] = (0, 0);
        }
        updated_count = 0;
        
        for (x, y) in entity_positions.iter() {
            term.move_cursor_to(*x as usize, *y as usize)?;
            term.write_line("@")?;
        }

        term.move_cursor_to(0, 0)?;
        term.write_line(&format!("{}", z))?;
    }

    reset(&mut term)
}

fn reset(term: &mut Term) -> Result<(), io::Error> {
    println!("\x1b[37m");
    term.clear_screen()?;
    term.move_cursor_to(0, 0)?;
    
    Ok(())
}