use std::io;
use std::{thread,time};
use CommandBuilder::Command;
use Timer::Timer as ClockTime;

pub mod Notification;
pub mod CommandBuilder;
pub mod Timer;

fn main() {
    const COMMAND_TRACK:&str = "track";

   
    let mut timer:ClockTime = ClockTime::new();
    
    println!("{:#?}",timer);
    'app_main_loop:loop {
        println!("rdy_>");
        
        let mut user_input:String = String::new();

        io::stdin().read_line(&mut user_input).expect("Err: Reading your input");
        
        let mut command:Command = Command::new();

        command.parse_user_input(&user_input);
        
        //Command
        let command_0 = match command.parts.get(0) {
            Some(word) => word,
            None => continue
        };

        //Sleep time
        match command.parts.get(1) {
            Some(num) => {
               let sleep_time:u64 =  match num.trim().parse() {
                    Err(_) => continue,
                    Ok(num) => num
                };
                timer.edit_duration(sleep_time);
            },
            None => continue
        };
        
        match command.parts.get(2) {
            Some(clock_type) => match clock_type.trim() {
                "minute" | "minutes" => { timer.edit_in_seconds(false);},
                "seconds" | "sec" | "secs" | "second" => { timer.edit_in_seconds(true);},
                _ => ()
            },
            None => continue
        };

        
        if let COMMAND_TRACK = command_0.trim() {
                timer.render_clock();
                let msg = format!("Start counting down: {} second(s)",timer.duration);
                
                Notification::notify(&msg);

                println!("\n_tracking");
                
                thread::sleep(time::Duration::from_secs(timer.duration ));
                let msg = format!("time of {} second(s) is over",timer.duration);
                
                Notification::notify(&msg);
                
                println!("_tracked \n");
            } 
            else 
            {
                break 'app_main_loop;
            
            };
    }
    

}

fn parse_user_input_to_parts(some_string:&str) -> (&str,&str)
{
    for (index, byte) in some_string.chars().enumerate() {
        if byte == ' ' {
           return (&some_string[..index] , &some_string[index+1..])
        }
    }

    (&some_string[..],&some_string[..])
}
