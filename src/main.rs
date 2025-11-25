mod instance;
mod downloadmc;

use std::io;
use std::process::Command;

fn main() {
    println!("Loading....");

    let version = Command::new("java")
        .arg("-version")
        .spawn();

    let launch = Command::new("java")
        .arg("-cp")
        .arg("minecraft.jar")
        .arg("net.minecraft.client.Minecraft")
        .spawn();

    println!("Welcome to LiteLauncher");
    println!("Checking Java version...");

    match version {
        Ok(mut child) => {
            child.wait().unwrap();
        }
        Err(e) => {
            println!("Failed to check Java version. Is it installed?: {}", e);
        }
    }


    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let user_input = user_input.trim();

    if user_input == "launch" {
        println!("Launching...");

        match launch {
            Ok(mut child) => {
                child.wait().unwrap();
            }
            Err(e) => {
                println!("Failed to launch. Is it installed?: {}", e);
            }
        }
    }
}
