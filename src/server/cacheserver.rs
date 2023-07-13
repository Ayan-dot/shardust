use crate::server::internal::lru::LRU;
use tonic::{transport::Server, Request, Response, Status};
struct CacheServerHandle {
    cache: LRU<u64, u64>,
    domain: String,
    server: Option<tonic::transport::Server>,
    address: String,
}

impl CacheServerHandle {
    fn new(url: String, address: String) -> Self {
        Self {
            domain: url,
            address: address,
            cache: LRU::new(100),
            server: None,
        }
    }
}
