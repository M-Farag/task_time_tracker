use notifica;

pub fn notify(message:&str) -> ()
{
    notifica::notify("Task started", message).expect("Err sending a notification");
}