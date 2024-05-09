// #[macro_use] extern crate rocket;

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![index])
// }
use financial_manage::b3::parse_file;
use calamine::{Reader, open_workbook, Xlsx, Data};

pub fn main() {
    parse_file("test.xlsx").unwrap();
}