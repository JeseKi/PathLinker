// #[test]
// fn creat_test () {
//     use libs::utils;
//     use libs::db;
// 
//     let conn = db::connect();
//     let url = utils::generate_random_url();
//     let file_name = "2024-3-11.xmind";
//     let path = "/home/Jese__Ki/Desktop/daily_notes/2024-3-11.xmind";
//     println!("url: {url}");
// 
//     let mut flag: Result<bool, rusqlite::Error> = Err(rusqlite::Error::InvalidQuery);  // 使用一个默认错误初始化
// 
//     match conn {
//         Ok(conn) => {
//             println!("Connected to SQLite database");
//             flag = db::crud::create_mapping(&conn, &file_name, &path, &url);
//         }
//         Err(err) => {
//             println!("Failed to connect to SQLite database: {:?}", err);
//             // 可以选择在这里设置一个特定的错误
//         }
//         _ => {
//             println!("Unknown error occurred");
//             // 可以选择在这里设置一个特定的错误
//         }
//     }
//     
//     // 因为 flag 是 Result 类型，我们直接在这个 match 语句处理它
//     match flag {
//         Ok(true) => {
//             println!("添加成功")
//         }
//         Ok(false) => {
//             println!("添加失败")
//         }
//         Err(err) => {
//             println!("出现错误: {:?}", err)
//         }
//     }
// }    

#[test]
fn read_test () {
    use libs::utils;
    use libs::crud;
    use libs::db;

    let conn = crud::connect_db();

    let mappings = db::base_crud::get_all_mappings(&conn);

    match mappings {
        Ok(mappings) => println!("{:?}", mappings),
        Err(err) => println!("{:?}", err),
    }
}