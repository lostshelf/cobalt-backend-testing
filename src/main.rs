use std::net::{IpAddr};
use std::fmt::{self, Formatter};
use chrono::{Date, Utc};
use clap::{App, arg};

struct User {
    name: String,
    id: usize,
    ip: IpAddr,
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
    let matches = App::new("Cobalt Backend Testing")
        .author("Hadi Faraz, <hadifaraz52@protonmail.com>")
        .version("0.1.0")
        .about("Sends a message to another computer with the power of TCP.")
        .arg(
            arg!(-a --address <ADDRESS> "IPv4 address to send message to")
                .required(false)
                .allow_invalid_utf8(false),
        );

    if let Some("address") = matches.value_of()
}