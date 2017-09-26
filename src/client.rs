
use ws;

pub struct Client {
    pub out: ws::Sender
}

impl ws::Handler for Client {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        self.out.send("Hello WebSocket")
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        println!("Got Message: {}", msg);

        self.out.close(ws::CloseCode::Normal)
    }
}
