use std::time;
use std::thread;
use std::process::Command;

use regex::Regex;
use clap::Parser;

const BATTERY: &str = "BAT0";
const LOW_THRESHOLD: u32 = 20;//%
const HIGH_THRESHOLD: u32 = 95;//%
const CHECK_INTERVAL: u64 = 60; //seconds
const NOTIFY_TIMEOUT: u32 = 10000; //ms

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args{
    #[arg(short, long, default_value_t=BATTERY.to_string())]
    battery: String,

    #[arg(short, long, default_value_t=LOW_THRESHOLD)]
    low_threshold: u32,
    
    #[arg(long, default_value_t=HIGH_THRESHOLD)]
    high_threshold: u32,

    #[arg(short, long, default_value_t=CHECK_INTERVAL)]
    check_interval: u64,

    #[arg(short, long, default_value_t=NOTIFY_TIMEOUT)]
    notify_timeout: u32,
}

fn send_notification(level: u32, notify_timeout: u32, title: &str){
    let level_string = format!("Battery level at {}%", level);

    Command::new("notify-send")
        .arg("-u")
        .arg("critical")
        .arg("-t")
        .arg(notify_timeout.to_string())
        .arg(title)
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
    let args = Args::parse();
    let wait_duration = time::Duration::from_secs(args.check_interval);
    let mut already_notified_low = false;
    let mut already_notified_high = false;
    loop{
        let battery_level = get_battery_level(&args.battery).unwrap();
        dbg!(battery_level);
        if battery_level < args.low_threshold && !already_notified_low {
            send_notification(battery_level, args.notify_timeout, "LOW BATTERY");
            already_notified_low = true;
        }else if battery_level > args.low_threshold{
            already_notified_low = false;
        }
        
        if battery_level >= args.high_threshold && !already_notified_high{
            println!("hello");
            send_notification(battery_level, args.notify_timeout, "BATTERY CHARGED");
            already_notified_high = true;
        }else if battery_level <= args.high_threshold{
            println!("hello 2");
            already_notified_high = false;
        }
        thread::sleep(wait_duration);
    }
}


