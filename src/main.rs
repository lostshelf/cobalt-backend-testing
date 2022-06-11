use std::net::{Ipv4Addr};
use std::fmt::{self, Formatter};
use chrono::{Date, Utc};
use clap::{Command, arg};
use rand::Rng;

struct User {
    name: String,
    id: usize,
    ip: Ipv4Addr,
}

impl Default for User {
    fn default() -> Self {
        User {
            name: String::from("host"),
            id: rand::thread_rng().gen(),
            ip: Ipv4Addr::new(127, 0, 0, 1),
        }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

struct Message {
    contents: String,
    id: usize,
    sender: User,
    date: Date<Utc>,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<{}> {}: {}", self.date, self.sender.name, self.contents)
    }
}

fn main() {
    let matches = Command::new("Cobalt Backend Testing")
        .author("Hadi Faraz, <hadifaraz52@protonmail.com>")
        .version("0.1.0")
        .about("Sends a message to another computer with the power of TCP.")
        .arg(
            arg!(-a --address <ADDRESS> "IPv4 address to send message to")
                .required(false)
                .allow_invalid_utf8(false),
        )
        .arg(
            arg!(-n --name <USERNAME> "Set your username for the application")
                .required(false)
                .allow_invalid_utf8(true),
        ).get_matches();


}