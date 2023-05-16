//todo this works on linux but the windows impl is broken

use getkey;
fn main() {
    println!("Press the Up Arrow to start.");
    loop {
        let key = getkey::getkey();
        match key {
            Ok(k) => if k == getkey::Key::Up {
                println!("Started!\n Finished!");
                return;
            }
            _ => {}
        }
        println!("Wrong Key!\nPress the Up Arrow to start.");
    }
}