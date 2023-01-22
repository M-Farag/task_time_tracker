use std::io;
use std::{thread,time};
use notifica;

fn main() {
    const COMMAND_TRACK:&str = "track";
    const COMMAND_EXIT:&str = "exit";
    const SIXTY:u64 = 60;

    'app_main_loop:loop {
        println!("rdy_>");
        
        let mut user_input:String = String::new();

        io::stdin().read_line(&mut user_input).expect("Err: Reading your input");
        
        let user_input = user_input.trim();
        let (command,sleep_time) = parse_user_input_to_parts(user_input);
        
        match command {
            COMMAND_TRACK => {
                let sleep_time:u64 = match sleep_time.trim().parse() {
                    Err(_) => continue,
                    Ok(num) => num
                };
                
                let msg = format!("Start counting down: {} minute(s)",sleep_time);
                
                match notifica::notify("Task started", &msg) {
                    Err(_) => continue,
                    Ok(msg) => msg
                }
                println!("\n_tracking");
                
                thread::sleep(time::Duration::from_secs(sleep_time * SIXTY));
                let msg = format!("time of {} minute(s) is over",sleep_time);
                
                match notifica::notify("Task ended", &msg) {
                    Err(_) => continue,
                    Ok(msg) => msg
                }
                println!("_tracked \n");
            },
            
            COMMAND_EXIT => {
              break 'app_main_loop;
            },

            _ => continue,
        };
    }
    

}

fn parse_user_input_to_parts(some_string:&str) -> (&str,&str)
{
    let parts = some_string.as_bytes();

    for (index, &byte) in parts.iter().enumerate() {
        if byte == b' ' {
           return (&some_string[..index] , &some_string[index+1..])
        }
    }

    (&some_string[..],&some_string[..])
}
