// first we load all crates that we need
extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate bytes;

// and load all modules
mod typedefs;
mod parser;
mod serializer;
mod service;

// now the compiler knows about all the sources we want to use, so lets start
// first setup the uses we need. Here we need just the uses for setting up the
// reactor and the proto pipeline server
use bytes::buf::BlockBuf;
use tokio_core::reactor::Core;
use tokio_proto::{server, Framed, pipeline};

// so we start off with the most obvious thing: Writing the main-function
fn main() {
    // first we need the reactor itself. Without a reactor, we can't do async stuff
    let mut reactor = Core::new().unwrap();
    // then we prepare the things we need from the server, namely a handle and the address
    let handle = reactor.handle();
    let address = "127.0.0.1:8888".parse().unwrap();

    // then we setup the server itself. We're not interested in the return value, the
    // server instance, because it's running and that's ok. The most important here
    // is the closure we pass by
    let _ = server::listen(&handle, address, move |stream| {
        // first we create a Framed object, our transport for this stream. A transport
        // is basically the stream itself. tokio_proto manages the Framing and all by
        // itself, we just provide some buffers, and a parser and serializer, that parse
        // and serialize our buffers from the stream for the service
        let stream_frame = Framed::new(
            stream,
            parser::Parser,
            serializer::Serializer,
            BlockBuf::default(),
            BlockBuf::default()
        );
        // and then we setup the service, that manages the protocol itself. Each frame, generated
        // by the previous Framed transport, gets processed by this service.
        let stream_service = service::PipeService {};
        
        // and then we setup and return a pipelined server, that manages our transport and the
        // service
        pipeline::Server::new(
            stream_service,
            stream_frame
        )
    }).unwrap();
    
    // and finally we tell the reactor to run an empty future, that never ends.
    reactor.run(futures::empty::<(), ()>()).unwrap();
}
