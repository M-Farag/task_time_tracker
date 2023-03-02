use std::io;
use std::{thread,time};
use crate::command_builder::Command;
use crate::timer::Timer as ClockTime;

pub mod notification;
pub mod command_builder;
pub mod timer;

fn main() {
    const COMMAND_TRACK:&str = "track";

   
    let mut timer:ClockTime = ClockTime::new();
    
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
                let msg = format!("Start tracking down: {} second(s)",timer.duration);
                
                notification::notify(&msg);

                println!("\n_tracking");
                
                thread::sleep(time::Duration::from_secs(timer.duration ));
                let msg = format!("time of {} second(s) is over",timer.duration);
                
                notification::notify(&msg);
                
                println!("_tracked \n");
            } 
            else 
            {
                break 'app_main_loop;
            
            };
    }
    

}
