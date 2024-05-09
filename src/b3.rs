use calamine::{open_workbook, Data, Reader, Xlsx, XlsxError};

struct Stock {
    product: String,
    institution: String,
    account: String,
    negotiation_code: String,
    company_cnpj: String,
    isin_code: String,
    stock_type: String,
    bookkeeper: String,
    quantity: i32,
    available_quantity: i32,
    unavailable_quantity: i32,
    reason: String,
    last_price: f64,
    updadate_date: f64,
}

struct Fii {

}

struct Tesouro_direto {}


pub fn parse_file(filepath: &str) -> Result<(), XlsxError> {
    let mut workbook: Xlsx<_> = open_workbook(filepath).unwrap();

    let sheets = workbook.sheet_names().to_owned();

    let expected_sheets = vec![String::from("Acoes"), String::from("Fundo de Investimento"), String::from("Tesouro Direto")];
    assert!(sheets.iter().all(|item| expected_sheets.contains(&item)));

    let sheet = workbook.worksheet_range("Acoes")?;

    let rows = sheet.rows();
    let headers = rows.clone().take(1).next().expect("No headers found");
    println!("{:?}", headers);

    for row in rows.skip(1) {
        println!("{:?}", row);
    }
    

    todo!();
}
