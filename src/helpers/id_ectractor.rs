use std::fmt::Error;

pub fn extract_id(vec_command_and_content: &Vec<&str>) -> Result<u32, Error> {
    let raw_id = match vec_command_and_content.get(1) {
        Some(id) => id,
        None => {
            println!("You need to write an ID after rm command!!!");
            return Err(Error);
        }
    };

    let _: u32 = match raw_id.trim().parse() {
        Ok(id) => return Ok(id),
        Err(_) => {
            println!("ID must be a valid number!!!");
            return Err(Error);
        }
    };
}