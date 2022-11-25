/*
    My attempt at csv parser
    2022-11-25  Sven Ponelat
*/



use std::fs::*;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;




pub fn csv_get_str_vec(file: &str, num_fields: usize, skip: i32, delimiter: &str) -> Result<Vec<Vec<String>>, String> {
    let mut ret: Vec<Vec<String>> = Vec::new();
    let mut line_counter = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(file) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            line_counter += 1;

            if line_counter > skip {
                // process line
                if let Ok(ip) = line {
                    let split = ip.split(delimiter);
                    let vec_str: Vec<&str> = split.collect();
                    let len = vec_str.len();
                    if len != num_fields {
                        let message = format!("Line {} in the file \"{}\" does not contain {} fields!",line_counter, file, num_fields);
                        return Err(message);
                    }

                    // convert the &str to Strings and make a vector
                    let mut to_be_pushed: Vec<String> = Vec::new(); 
                    for s in vec_str {
                        to_be_pushed.push(s.to_string());
                    }

                    // add the new vector to the collection of vectors
                    ret.push(to_be_pushed);
                }
                else {
                    let message = format!("Error in reading line {} in the file \"{}\"",line_counter, file);
                    return Err(message);
                }
            }
            else {
                continue;
            }
        }
    }
    else {
        let message = format!("Error in opening the file \"{}\"",file);
        return Err(message);
    }

    Ok(ret)
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}












// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  Tests  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@         @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

#[cfg(test)]
mod tests {
    use super::*;

    
    // #[ignore]
    #[test]
    fn t001_first() {
        // let mut veccy: Vec<Vec<String>> = Vec::new();

        let source = "./test/some-documents/small.csv";
        let destination = "./test/work.csv";
        copy(source,destination).expect("Failed to copy");
        let res_veccy = csv_get_str_vec(destination,6, 2,";");
        remove_file(destination).expect("Cleanup test failed");

        assert!(res_veccy.is_ok());
    }

    // #[ignore]
    #[test]
    fn t002_failures() {

    }


















} //end of tests


