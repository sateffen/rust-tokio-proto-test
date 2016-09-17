use typedefs::Frame;
use bytes::{BlockBuf, MutBuf};
use tokio_proto::{Serialize, pipeline};

// this struct is our serializer. This will serialize the answer from
// the service and send it to the stream (TcpStream).
pub struct Serializer;

// and to actually work, we have to impement the serialize trait
impl Serialize for Serializer {
    // first we define the type we want to consume. tokio_proto will pass
    // this to us after the service generated the data.
    type In = Frame;

    // and we have to implement the serialize method. Here we match the result
    // from the service, and use the match. To write to the buffer we have to
    // write to given BlockBuffer
    fn serialize(&mut self, frame: Self::In, buf: &mut BlockBuf) {
        match frame {
            pipeline::Frame::Message(msg) => buf.write_str(&msg),
            pipeline::Frame::Body(body) => println!("BODY{:?}", body),
            pipeline::Frame::MessageWithBody(_,_) => println!("GOT MESSAGE WITH BODY"),
            pipeline::Frame::Error(e) => println!("Error {:?}", e),
            pipeline::Frame::Done => println!("DONE"),
        }
    }
}