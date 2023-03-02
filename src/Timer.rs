#[derive(Debug)]
pub struct  Timer { 
    pub duration: u64,
    pub in_seconds: bool 
}

impl Timer {
   pub fn new() -> Self
   {
    Self { duration: 60, in_seconds: true }
   }

   pub fn render_clock(&mut self) -> ()
   {
        if self.in_seconds == false {
            self.duration = self.duration * 60;
        }
   }

   pub fn edit_duration(&mut self, duration:u64) -> ()
   {
        self.duration = duration;
   }

   pub fn edit_in_seconds(&mut self, in_seconds:bool) -> ()
   {
        self.in_seconds = in_seconds;
   }
}