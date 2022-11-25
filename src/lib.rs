/*
    My attempt at csv parser
    2022-11-25  Sven Ponelat
*/



use std::fs::*;
use std::io::prelude::*;






















// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@  Tests  @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@
// @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@         @@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@

#[cfg(test)]
mod tests {
    use super::*;

    
    // #[ignore]
    #[test]
    fn t001() {
        let source = "./test/some-documents/good_csv.csv";
        let destination = "./test/work.csv";
        copy(source,destination).expect("Failed to copy");



        remove_file(destination).expect("Cleanup test failed");
    }

    // #[ignore]
    #[test]
    fn t002() {

    }


















} //end of tests


