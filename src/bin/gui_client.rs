use iced::widget::{button, column, text};
use iced::{Alignment, Element, Sandbox, Settings};
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() -> iced::Result {
    SmartSocket::run(Settings {
        window: iced::window::Settings {
            size: (300, 300),
            ..Default::default()
        },
        ..Default::default()
    })
}

struct SmartSocket {
    state: String,
    power: String,
    stream: TcpStream,
    buf: Vec<u8>,
}

#[derive(Debug, Clone)]
enum Message {
    TurnOn,
    TurnOff,
    UpdateState,
}

// TCP commands to Smart socket
const TURN_OFF: i32 = 0;
const TURN_ON: i32 = 1;
const GET_STATE: i32 = 2;
const GET_POWER: i32 = 3;

impl Sandbox for SmartSocket {
    type Message = Message;

    fn new() -> Self {
        SmartSocket {
            state: String::from("Unknown"),
            power: String::from("Unknown"),
            stream: TcpStream::connect("127.0.0.1:5555").expect("connection failed"),
            buf: vec![0u8; 128],
        }
    }

    fn title(&self) -> String {
        String::from("Smart socket")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TurnOn => {
                self.state = format!("State: {}", self.do_request(TURN_ON));
                self.power = format!("Power: {}", self.do_request(GET_POWER));
            }

            Message::TurnOff => {
                self.state = format!("State: {}", self.do_request(TURN_OFF));
                self.power = format!("Power: {}", self.do_request(GET_POWER));
            }

            Message::UpdateState => {
                self.state = format!("State: {}", self.do_request(GET_STATE));
                self.power = format!("Power: {}", self.do_request(GET_POWER));
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            text(&self.state).size(30),
            text(&self.power).size(30),
            button("Turn On").on_press(Message::TurnOn),
            button("Turn Off").on_press(Message::TurnOff),
            button("Update State").on_press(Message::UpdateState),
        ]
        .padding(50)
        .align_items(Alignment::Center)
        .into()
    }
}

impl SmartSocket {
    fn do_request(&mut self, command: i32) -> String {
        self.stream
            .write_all(&command.to_be_bytes())
            .expect("fail to request");
        let n = self.stream.read(&mut self.buf).expect("fail to get reply");
        let reply = String::from_utf8_lossy(&self.buf[..n]).into_owned();
        reply
    }
}
