use libs::hard_link_create;
#[test]
fn try_create () {
    let path = "/home/Jese__Ki/Downloads/icon.png";
    let link = hard_link_create(path);
    match link {
        Ok(link) => {
            println!("{}", link);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}