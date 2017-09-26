
use std::fmt;

use ws;
use bytes::{ BufMut };
use protobuf::{ CodedOutputStream, Message };
use nuro_sc2_proto::sc2api::Request;

pub enum ClientErr {
    SendFailed
}

impl fmt::Debug for ClientErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ClientErr::SendFailed => write!(f, "send failed")
        }
    }
}

pub struct Client {
    pub out:        ws::Sender,
}

impl ws::Handler for Client {
    fn on_message(&mut self, _: ws::Message) -> ws::Result<()> {
        println!("received response");

        Ok(())
    }
}

impl Client {
    // TODO: error specificity
    pub fn call_api(&self, req: Request) -> Result<(), ClientErr> {
        let buf = Vec::new();
        let mut writer = buf.writer();

        {
            let mut cos = CodedOutputStream::new(&mut writer);

            match req.write_to(&mut cos) {
                Ok(_) => { }
                Err(_) => return Err(ClientErr::SendFailed)
            }
            match cos.flush() {
                Ok(_) => { }
                Err(_) => return Err(ClientErr::SendFailed)
            }
        }

        match self.out.send(ws::Message::Binary(writer.into_inner())) {
            Ok(_) => {
                println!("sent payload");
                Ok(())
            }
            Err(_) => Err(ClientErr::SendFailed)
        }
    }
}
