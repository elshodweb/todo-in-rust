pub fn extract_command (command: &str)-> Vec<&str>{

    let mut response = vec![];
    let trimed = command.trim();
    match trimed.split_once(" ") {
        None => {
            response.push(trimed);
        },
        Some((cmd,rest)) => {
            response.push(cmd);
            response.push(rest.trim());
        }
    }
    response
}