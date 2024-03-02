use crate::start_line::StartLine;


pub struct Message {
    start_line: StartLine,
    headers: Vec<(String, String)>,
    body: String
}
