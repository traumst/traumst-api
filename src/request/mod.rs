use crate::{response, router};

pub fn process(buffer: [u8; 512]) -> String {
    let http_request = std::str::from_utf8(&buffer).unwrap();
    let routing_result = router::handle_request(http_request);
    response::generate_for(routing_result)
}