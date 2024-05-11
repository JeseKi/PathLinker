use rusqlite::Result;
use libs::crud::connect_db;
use libs::db::folder_base_crud as folder_crud;
#[test]
fn drop_table() -> Result<(), rusqlite::Error> {
    let conn = connect_db(); // 假设这是你的数据库连接函数

    // 执行删除表的 SQL 命令
    conn.execute("DROP TABLE IF EXISTS mapping_folders", [])?;

    println!("Table 'mapping_folders' has been dropped.");

    Ok(())
}

#[test]
fn get_folders() {
    let conn = connect_db();

    let folders = folder_crud::get_all_folders(&conn);
    println!("Folders: {:?}", folders);
}