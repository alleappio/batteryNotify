use std::process::Command;

fn send_notification(level: i8){
    let level_string = format!("Battery level at {}%", level);

    Command::new("notify-send")
        .arg("-u")
        .arg("critical")
        .arg("-t")
        .arg("10000")
        .arg("LOW BATTERY")
        .arg(level_string)
        .spawn()
        .expect("Failed to execute program");
}

fn main(){
    send_notification(20);
}
