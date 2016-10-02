use typedefs::Frame;
use bytes::{Buf};
use bytes::buf::BlockBuf;
use tokio_proto::{Parse, pipeline};
use std::io::{Error, ErrorKind};
use std::str;

// and here we define the parser. The parser will receive the incoming BlockBuffer
// and generates a Frame based on it
pub struct Parser;
impl Parse for Parser {
    // here we tell, that we will output a Frame
    type Out = Frame;

    // and we need to implement the parse method. Here we take the incoming
    // BlockBuffer, and generate a Frame-option based on it. In this case
    // we transform the buffer to a string, and pass a FrameMessage on, if
    // converting is successful.
    fn parse(&mut self, buf: &mut BlockBuf) -> Option<Self::Out> {
        // first we check whether the buffer is compact. If it isn't we can't
        // work with it like we want it (because it might be chunked some methods
        // won't work I guess). To make sure it works, we make it compact
        if !buf.is_compact() {
            buf.compact();
        }

        // then we do some more steps to convert BlockBuf -> [u8] and clean up
        // the BlockBuffer. We have to "flush" (shift or drop) the BlockBuf, because
        // otherwise the next call to parse will contain this bytes as well. We have
        // to consume the bytes, not only look at them.
        let buf_len = buf.len();
        let buf_bytesshift = buf.shift(buf_len);
        let buf_bytesbuf = buf_bytesshift.buf();
        let buf_bytes = buf_bytesbuf.bytes();
        // now buf_bytes is an [u8], we can work with that

        // and we work by that by converting it to an utf8 string. If converting is
        // successful, we pipe out the string as message, otherwise we send out an error
        match str::from_utf8(buf_bytes) {
            Ok(s) => Some(pipeline::Frame::Message(s.to_string())),
            Err(_) => Some(pipeline::Frame::Error(Error::new(ErrorKind::Other, "invalid string"))),
        }
    }
}