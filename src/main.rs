use std::process::Command;
use regex::Regex;

fn send_notification(level: u32){
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

fn get_battery_level(battery: &str) -> Option<u32>{
    let battery_string = format!("/org/freedesktop/UPower/devices/battery_{}", battery);
    let command = Command::new("upower")
                        .arg("-i")
                        .arg(battery_string)
                        .output()
                        .expect("Failed to execute program");

    let total_string = match String::from_utf8(command.stdout){
        Ok(v) => v,
        Err(e) => panic!("aiutooooooo"),
    };

    let re = Regex::new(r"percentage:\s+(\d+)%").unwrap();
    if let Some(results) = re.captures(&total_string) {
        results[1].parse::<u32>().ok()
    }else{
        None
    }
}

fn main(){
    let battery_level = get_battery_level("BAT1").unwrap();
    send_notification(battery_level);
}


