
extern crate diesel;
extern crate dotenv;
extern crate walkdir;

use std::env;
use dotenv::dotenv;
use std::path::{Path,PathBuf};
use self::walkdir::WalkDir;
use diesel::prelude::*;
use diesel::dsl::sql;
use diesel::pg::PgConnection;

///
/// read the directories from command line arguments and start import
///
fn main() {
    let mut dirs:Vec<PathBuf> = env::args().filter(|x| filterpath(x)).map(|x| PathBuf::from(&x)).collect();
    if dirs.is_empty() {
        dirs.push(PathBuf::from("."));
    } else {
    let conn = get_conn();
    let setup = sql::<bool>("DELETE FROM excel2pg;");
    setup.execute(&conn).expect("Can't delete data");
    for dir in dirs {
        for entry in WalkDir::new(dir).follow_links(true).into_iter().filter_map(|e| e.ok()) {
            let f_name = entry.file_name().to_string_lossy();
            if f_name.ends_with(".xlsx") || f_name.ends_with(".xls") || f_name.ends_with(".xlsm") {
                println!("Process {}",f_name);
                excel2pg::import_file(&entry.path(),&conn);
            }
        }
    }
    }
}

fn filterpath(d:&str) -> bool {
    let p = Path::new(d);
    p.exists() && p.is_dir()
}

fn get_conn() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting"))
}

#[test]
fn test_filter(){
    assert!(filterpath("."));
    assert!(!filterpath("/öäüß"));
}
