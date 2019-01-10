#[macro_use]
extern crate diesel;
extern crate calamine;

mod models;
mod schema;

use calamine::{open_workbook_auto, DataType, Reader};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::path::Path;

use self::models::Excel2pg;

pub fn import_file(path: &Path, conn:&PgConnection) {
    //
    // iterate over all sheets and push them to db
    //
    let mut workbook = open_workbook_auto(path).expect("Cannot open file");
    let sheets = workbook.sheet_names().to_owned();
    let filename = std::fs::canonicalize(path).unwrap();
    for s in sheets {
        if let Some(Ok(range)) = workbook.worksheet_range(&s) {
            //
            // now we have a sheet
            //
            let sheetname = String::from(s);
            let (rows, cols) = range.get_size();
            assert_eq!(
                range.used_cells().count(),
                range.rows().flat_map(|r| r.iter().filter(|&c| c != &DataType::Empty)).count()
            );
            println!("...filename, sheetname, rownumber {}{}{}",path.to_str().unwrap(),sheetname,rows);
            let mut count = 0;
            let mut batch = Vec::<Excel2pg>::with_capacity(500);
            let mut inserted_rows = 0;
            for (row_idx, row) in range.rows().enumerate() {
                let dbrow = Excel2pg {
                    file:  filename.to_str().unwrap().to_string(),
                    sheet: sheetname.clone(),
                    fdate: chrono::naive::NaiveDateTime::from_timestamp(0,0),
                    idate: chrono::naive::NaiveDateTime::from_timestamp(0,0),
                    s001:  "".to_string(),
                    s002:  "".to_string(),
                    s003:  "".to_string(),
                };
                batch.push(dbrow);
                count += 1;
                inserted_rows += 1;
                if 0 == count % 500 || inserted_rows==rows {
                    diesel::insert_into(schema::excel2pg::table).values(&batch).execute(conn).expect("insert statement failed");
                    count = 0;
                    batch.clear();
                }
            }
            assert_eq!(inserted_rows,rows);
        }
    }
}
