use std::io;
use std::thread::sleep;
use std::time::{Duration, Instant};
use chrono::{DateTime, Local, Duration as ChrDuration};
use std::cmp::Ordering;

struct Clock {
    start: DateTime<Local>,
    duration: u32,
    end: DateTime<Local>,
}

fn install_client() {
    let na_client_url = "https://lol.secure.dyn.riotcdn.net/channels/public/x/installer/current/live.na.exe";
    let response = reqwest::blocking::get(na_client_url)
        .expect("Failed to download installer");
    let client = response.bytes().expect("response invalid");
    let _ = std::fs::write("LoLClientInstaller.exe", &client);

    std::process::Command::new(r".\LoLClientInstaller.exe")
        .arg("--skip-to-install")
        .spawn()
        .expect("Failed to execute process");
}

fn set_clock() -> Clock {
    let mut start_time = chrono::Local::now();

    println!("Enter clock duration in minutes: ");
    let mut duration = String::new();
    io::stdin()
        .read_line(&mut duration)
        .expect("Failed to read line");
    let duration: u32 = duration.trim().parse().expect("Invalid number");
    let end_time = start_time + ChrDuration::minutes(duration as i64);

    let clock = Clock{
        start: start_time,
        duration,
        end: end_time,
    };

    let formatted_start_time = start_time.format("%Y-%m-%d %H:%M:%S").to_string();
    let formatted_end_time = end_time.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("Clock set. Start time: {:?}, End time: {:?}", formatted_start_time, formatted_end_time);

    return clock;
}

fn check_clock(clock: &Clock) -> bool {
    match clock.end.cmp(&chrono::Local::now()) {
        Ordering::Less => return true,
        Ordering::Greater => return false,
        Ordering::Equal => return false,
    }
}

fn uninstall_league() {
    /* Needs to turn off riot client for it to work
    std::process::Command::new(r"C:\Riot Games\Riot Client\RiotClientServices.exe")
        .arg("--uninstall-product=league_of_legends")
        .arg("--uninstall-patchline=live")
        .spawn()
        .expect("Failed to uninstall");
    */
    // nuclear option
    let _= std::fs::remove_dir_all(r"C:\Riot Games\League of Legends");
}

fn main() {
    install_client();
    let mut clock = set_clock();

    let interval = Duration::from_secs(1);
    let mut next_time = Instant::now() + interval;
    loop {
        if check_clock(&clock) {
            break;
        }
        sleep(next_time - Instant::now());
        next_time += interval;
    }
    uninstall_league();
}
