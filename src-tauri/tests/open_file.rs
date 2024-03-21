use libs::db;
use open;

#[test]
fn open_file () {

    let conn = db::connect();

    let mut my_mapping: Vec<db::base_crud::Mapping> = vec![];

    match conn {
        Ok(conn) => {
            println!("Connected to SQLite database");
            let mappings  = db::base_crud::get_all_mappings(&conn);
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
            if let Err(e) = open::that(&mapping.path) { // 根据数据库获取的path打开文件
                eprintln!("Failed to open the file: {}", e);
            }
        }
        None => {
            println!("No mapping found");
        }
    }
}