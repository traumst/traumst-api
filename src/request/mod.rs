use crate::{response, router};

pub fn process(buffer: [u8; 512], bytes_read: usize) -> String {
    let http_request = std::str::from_utf8(&buffer[..bytes_read]).unwrap();
    let routing_result = router::handle_request(http_request);
    response::generate_for(routing_result)
}