use std::net::{Ipv4Addr};
use std::fmt::{self, Formatter};
use std::str::FromStr;
use chrono::{DateTime, Utc};
use clap::{Command, arg, ArgMatches};
use rand::Rng;
use pnet::datalink;

struct User {
    name: String,
    ip: Ipv4Addr,
}

impl Default for User {
    fn default() -> Self {
        User {
            name: String::from("host"),
            ip: Ipv4Addr::new(127, 0, 0, 1),
        }
    }
}

impl From<ArgMatches> for User {
    fn from(m: ArgMatches) -> Self {
        User {
            name: m.value_of("name").unwrap_or_else(|| {
                "host"
            }).to_string(),
            ip: Ipv4Addr::new(127, 0, 0, 1), // TODO: Find a way to
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
    date: DateTime<Utc>
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<{}> {}: {}", self.date.time(), self.sender.name, self.contents)
    }
}

fn main() {
    let matches=Command::new("Cobalt Backend Testing")
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

    let _user = User::from(matches);

    let _target: Ipv4Addr = Ipv4Addr::from_str(matches.value_of("address").unwrap_or_else(|| {
        "127.0.0.1"
    })).unwrap_or_else(|| {
        Ipv4Addr::new(127, 0, 0, 1)
    });


}