use typedefs::{FrameMessage, FrameBody, FrameError};
use futures::{Future, Async, finished};
use futures::stream::Empty;
use tokio_service::Service;
use tokio_proto::pipeline;

// This service is used to manage our protocol server. It'll receive each
// message from the frames, so we can actually work with them.
pub struct PipeService;
// but to do so, our service has to implement tokio_service::Service
impl Service for PipeService {
    // first we setup the needed types
    type Request = FrameMessage;
    // The Response type is every time a pipeline::Message
    type Response = pipeline::Message<FrameMessage, Empty<FrameBody, FrameError>>;
    type Error = FrameError;
    // a special thing is this future. We have to box it, because we can't determine
    // the actual size of the Self::Response at compile type
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    // then we define the actual service call function. This handles the service action
    fn call(&self, data: Self::Request) -> Self::Future {
        // we don't want to do anything secial, so we simply pipe the message back out.
        // if you want to to anything special in your service, you have to do it here
        finished::<Self::Response, Self::Error>(
            // here we could use Message::WithBody as well, but this bugs our pipeline.
            // after using this line, the Parser will get called in an endless loop.
            // I'll try to check what's wrong and update this part, but currently don't
            // use it
            // pipeline::Message::WithBody(data, futures::stream::empty::<(), Error>())
            
            // but you can use this. We send data without a body, which works as expected
            pipeline::Message::WithoutBody(data)
        ).boxed()
    }
    
    // and we need to define a poll_ready function. As far as I got it it's telling when
    // the service can be used. In our case the service is ready every time, but if the
    // service works with a limited resource, you might change this function
    fn poll_ready(&self) -> Async<()> {
        Async::Ready(())
    }
}