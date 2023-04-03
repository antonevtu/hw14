use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

struct SmartSocket {
    state: State,
    power: f32,
}

enum State {
    On,
    Off,
}

fn main() {
    let mut socket = SmartSocket {
        state: State::Off,
        power: 100.0,
    };

    let listener = TcpListener::bind("127.0.0.1:5555").expect("bind failed");
    println!("server started at 127.0.0.1:5555");

    while let Some(stream) = listener.incoming().next() {
        if stream.is_err() {
            continue;
        }

        let stream = stream.unwrap();
        let peer = stream.peer_addr();
        println!("connected: {:?}", peer);
        process_stream(stream, &mut socket);
        println!("disconnected: {:?}", peer);
    }
}

fn process_stream(mut stream: TcpStream, socket: &mut SmartSocket) {
    let mut buf = [0u8; 4];
    loop {
        if stream.read_exact(&mut buf).is_err() {
            break;
        }

        let request = u32::from_be_bytes(buf);
        println!("request: {request}");

        let reply = match request {
            0 => socket.turn_off(),
            1 => socket.turn_on(),
            2 => socket.get_state(),
            3 => socket.get_power(),
            _ => unknown_request(),
        };

        if stream.write_all(reply.as_bytes()).is_err() {
            break;
        }
    }
}

impl SmartSocket {
    fn turn_off(&mut self) -> String {
        println!("received command: turn off");
        self.state = State::Off;
        String::from("turned off")
    }

    fn turn_on(&mut self) -> String {
        println!("received command: turn on");
        self.state = State::On;
        String::from("turned on")
    }

    fn get_power(&self) -> String {
        println!("received command: get_power");
        match &self.state {
            State::On => format!("{} W", self.power),
            State::Off => format!("{} W", 0),
        }
    }

    fn get_state(&self) -> String {
        println!("received command: get_state");
        match &self.state {
            State::On => String::from("turned on"),
            State::Off => String::from("turned off"),
        }
    }
}

fn unknown_request() -> String {
    println!("received command: unknown");
    String::from("unknown command")
}
