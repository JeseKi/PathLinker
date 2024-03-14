
use libs::crud;
#[test]
fn read_test () {
    let conn = crud::connect_db();
    let mappings = crud::get_mappings(&conn);

    println!("{:?}", mappings);
    let url = "pathlinker://NjqChOseJV";
    let path = crud::get_filepath_by_url(&conn, &url);
    println!("path:{path}")
}