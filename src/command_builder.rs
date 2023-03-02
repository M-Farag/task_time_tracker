#[derive(Debug)]
pub struct Command {
    pub parts:Vec<String>
}


impl Command {
    pub fn new() ->Self
    {
        Self { parts: Vec::new() }
    }

    pub fn parse_user_input(&mut self,user_input:&String) -> ()
    {
        let mut start_index = 0;
        for (end_index, letter) in user_input.chars().enumerate() {
            if letter == ' ' {
                self.parts.push(String::from(&user_input[start_index..end_index]));
                start_index = end_index + 1;
            }
        }
        self.parts.push(String::from(&user_input[start_index..]));
    }
}