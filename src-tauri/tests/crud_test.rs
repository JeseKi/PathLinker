#![cfg(test)]
use libs::{crud, utils};

#[test]
fn test() {
    let conn = crud::connect_db();
    let my_query = "pathlinker://lrULTffYQ3";
    let my_path = crud::get_filepath_by_url(&conn, my_query);
    println!("{:?}, len: {}", my_path, my_path.len())
}