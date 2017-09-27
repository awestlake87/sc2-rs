
use tungstenite::client::client;
use tungstenite::protocol::WebSocket;
use url::Url;

use super::{ Result, Error };

pub struct Client {
}

impl Client {
}

/*use ws;
use bytes::{ BufMut };
use protobuf::{ CodedOutputStream, Message };
use sc2_proto::sc2api::Request;

use super::{ Result, Error };

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
    pub fn call_api(&self, req: Request) -> Result<()> {
        let buf = Vec::new();
        let mut writer = buf.writer();

        {
            let mut cos = CodedOutputStream::new(&mut writer);

            match req.write_to(&mut cos) {
                Ok(_) => { }
                Err(_) => return Err(Error::WebsockSendFailed)
            }
            match cos.flush() {
                Ok(_) => { }
                Err(_) => return Err(Error::WebsockSendFailed)
            }
        }

        match self.out.send(ws::Message::Binary(writer.into_inner())) {
            Ok(_) => {
                println!("sent payload");
                Ok(())
            }
            Err(_) => Err(Error::WebsockSendFailed)
        }
    }
}*/
