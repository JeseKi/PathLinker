#[test]
fn creat_test () {
    use libs::utils;
    use libs::db;

    let conn = db::connect();
    let url = utils::generate_random_url();
    let file_name = "2024-3-11.xmind";
    let path = "/home/Jese__Ki/Desktop/daily_notes/2024-3-11.xmind";
    println!("url: {url}");

    let mut flag: Result<bool, rusqlite::Error> = Err(rusqlite::Error::InvalidQuery);  // 使用一个默认错误初始化

    match conn {
        Ok(conn) => {
            println!("Connected to SQLite database");
            flag = db::crud::create_mapping(&conn, &file_name, &path, &url);
        }
        Err(err) => {
            println!("Failed to connect to SQLite database: {:?}", err);
            // 可以选择在这里设置一个特定的错误
        }
        _ => {
            println!("Unknown error occurred");
            // 可以选择在这里设置一个特定的错误
        }
    }
    
    // 因为 flag 是 Result 类型，我们直接在这个 match 语句处理它
    match flag {
        Ok(true) => {
            println!("添加成功")
        }
        Ok(false) => {
            println!("添加失败")
        }
        Err(err) => {
            println!("出现错误: {:?}", err)
        }
    }
}    

#[test]
fn read_test () {
    use libs::utils;
    use libs::db;

    let conn = db::connect();

    let mut my_mapping: Vec<db::crud::Mapping> = vec![];

    match conn {
        Ok(conn) => {
            println!("Connected to SQLite database");
            let mappings  = db::crud::get_all_mappings(&conn);
            println!("{:?}", mappings);
            match mappings {
                Ok(mappings_vec) => {
                    // 现在 mappings_vec 是 Vec<Mapping> 类型，你可以对它进行索引操作
                    if !mappings_vec.is_empty() {
                        my_mapping.push(mappings_vec[1].clone());
                    } else {
                        println!("No mappings found");
                    }
                },
                Err(e) => {
                    println!("Error getting mappings: {:?}", e);
                },
            }
            
        }
        Err(err) => {
            println!("Failed to connect to SQLite database: {:?}", err);
            // 可以选择在这里设置一个特定的错误
        }
        _ => {
            println!("Unknown error occurred");
            // 可以选择在这里设置一个特定的错误
        }
    }
    println!("mappings: {:?}", my_mapping.get(0));

    match my_mapping.get(0) {
        Some(mapping) => {
            println!("Mapping found: {:?}", mapping);
            println!("Mapping url: {}", mapping.url);
            println!("Mapping path: {}", mapping.path);
        }
        None => {
            println!("No mapping found");
        }
    }
}