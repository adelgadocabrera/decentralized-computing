// use std::net::SocketAddr;
use tonic::{service::Interceptor, Request, Status};

#[derive(Clone)]
pub struct ClientAddressInterceptor {}

impl ClientAddressInterceptor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Interceptor for ClientAddressInterceptor {
    fn call(&mut self, request: Request<()>) -> Result<Request<()>, Status> {
        let _client_addr = request.remote_addr().unwrap();

        Ok(request)
    }
}
