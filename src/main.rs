extern crate irc;

use irc::client::prelude::*;

fn main() {
    let server = IrcServer::new("config.json").unwrap();
    server.identify().unwrap();
    for message in server.iter() {
        let message = message.unwrap(); // We'll just panic if there's an error.
        println!("{}", message.into_string());
        match message.command {
            Command::PRIVMSG(ref target, ref msg) => if target.starts_with("#") {
                server.send_privmsg(target, "Hi!").unwrap();
            },
            _ => (),
        }
    }
}
