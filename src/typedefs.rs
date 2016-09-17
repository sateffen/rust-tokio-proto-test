use std::io::Error;
use std::string::String;
use tokio_proto::pipeline;

// here we define just some types. These types are just to simplify the code,
// so it's simpler to read.
pub type FrameMessage = String;
pub type FrameError = Error;
pub type FrameBody = ();
pub type Frame = pipeline::Frame<FrameMessage, FrameBody, Error>;