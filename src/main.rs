extern crate websocket;
extern crate clap;

use std::str::from_utf8;
use websocket::client::request::Url;
use websocket::{Client, Message, Sender, Receiver};
use websocket::message::Type;
use clap::{Arg, App, SubCommand};

fn main() {
  let matches = App::new("websocket client")
              .version("1.0")
              .author("Andi Miller <andi at andimiller.net>")
              .about("is a pipeable websocket client")
              .arg(Arg::with_name("v")
                 .short("v")
                 .multiple(true)
                 .help("Sets the level of verbosity"))
              .subcommand(SubCommand::with_name("connect")
                    .about("connects to a websocket as a client")
                    .arg(Arg::with_name("URL")
                    .required(true)
                    .help("URL to connect to")))
              .get_matches();

  if let Some(matches) = matches.subcommand_matches("connect") {
    let addr = matches.value_of("URL").unwrap();
    let agent = "rust-websocket";

    println!("Using fuzzingserver {}", addr);
    println!("Using agent {}", agent);

    // websockets
    let ws_uri = Url::parse(&addr[..]).unwrap();
    let request = Client::connect(ws_uri).unwrap();
    let response = request.send().unwrap();

    match response.validate() {
      Ok(()) => (),
      Err(e) => {
        println!("{:?}", e);
      }
    }
    let (mut sender, mut receiver) = response.begin().split();


    for message in receiver.incoming_messages() {
      let message: Message = match message {
        Ok(message) => message,
        Err(e) => {
          println!("Error: {:?}", e);
          let _ = sender.send_message(&Message::close());
          break;
        }
      };

      match message.opcode {
        Type::Text => {
          let response = from_utf8(&*message.payload).unwrap();
          println!("{:?}", response);
          //sender.send_message(&response).unwrap();
        }
        Type::Binary => {
          sender.send_message(&Message::binary(message.payload)).unwrap();
        }
        Type::Close => {
          let _ = sender.send_message(&Message::close());
          break;
        }
        Type::Ping => {
          sender.send_message(&Message::pong(message.payload)).unwrap();
        }
        _ => (),
      }
    }
  }
}
