use calamine::{open_workbook, Data, DataType, Reader, Xlsx, XlsxError};

#[derive(Debug)]
pub enum InvestmentType {
    Stock(Stock),
    Fii(Fii),
    TreasuryDirect(TreasuryDirect),
}

#[derive(Debug)]
pub struct Stock {
    product: String,
    institution: String,
    account: String,
    negotiation_code: String,
    company_cnpj: String,
    isin_code: String,
    stock_type: String,
    bookkeeper: String,
    quantity: i64,
    available_quantity: i64,
    unavailable_quantity: i64,
    reason: String,
    last_price: f64,
    updated_value: f64,
}

#[derive(Debug)]
pub struct Fii {
    product: String,
    institution: String,
    account: String,
    negotiation_code: String,
    fund_cnpj: String,
    isin_code: String,
    stock_type: String,
    administrator: String,
    quantity: i64,
    available_quantity: i64,
    unavailable_quantity: i64,
    reason: String,
    last_price: f64,
    updated_value: f64,
}

#[derive(Debug)]
pub struct TreasuryDirect {
    product: String,
    institution: String,
    isin_code: String,
    indexer: String,
    deadline: String,
    quantity: f64,
    available_quantity: f64,
    unavailable_quantity: f64,
    reason: String,
    applied_value: f64,
    brute_value: f64,
    liquid_value: f64,
    updated_value: f64,
}

#[derive(Debug)]
pub struct ParsedFile {
    stocks: Vec<Stock>,
    total_value_stocks: f64,
    fiis: Vec<Fii>,
    total_value_fiis: f64,
    treasury_directs: Vec<TreasuryDirect>,
    total_value_treasury_directs: f64,
}

pub fn parse_unavailable_quantity(unavailable_quantity: Data) -> i64 {
    match unavailable_quantity {
        Data::Int(value) => value,
        Data::Float(value) => value as i64,
        _ => 0,
    }
}

pub fn parse_stock(row: &[Data]) -> Option<Stock> {
    let stock = Stock {
        product: row[0].as_string()?,
        institution: row[1].as_string()?,
        account: row[2].as_string()?,
        negotiation_code: row[3].as_string()?,
        company_cnpj: row[4].as_string()?,
        isin_code: row[5].as_string()?,
        stock_type: row[6].as_string()?,
        bookkeeper: row[7].as_string()?,
        quantity: row[8].as_i64()?,
        available_quantity: row[9].as_i64()?,
        unavailable_quantity: parse_unavailable_quantity(row[10].clone()),
        reason: row[11].as_string()?,
        last_price: row[12].as_f64()?,
        updated_value: row[13].as_f64()?,
    };
    Some(stock)
}

pub fn parse_fii(row: &[Data]) -> Option<Fii> {
    let fii = Fii {
        product: row[0].as_string()?,
        institution: row[1].as_string()?,
        account: row[2].as_string()?,
        negotiation_code: row[3].as_string()?,
        fund_cnpj: row[4].as_string()?,
        isin_code: row[5].as_string()?,
        stock_type: row[6].as_string()?,
        administrator: row[7].as_string()?,
        quantity: row[8].as_i64()?,
        available_quantity: row[9].as_i64()?,
        unavailable_quantity: parse_unavailable_quantity(row[10].clone()),
        reason: row[11].as_string()?,
        last_price: row[12].as_f64()?,
        updated_value: row[13].as_f64()?,
    };
    Some(fii)
}

pub fn parse_treasury_direct(row: &[Data]) -> Option<TreasuryDirect> {
    let treasury_direct = TreasuryDirect {
        product: row[0].as_string()?,
        institution: row[1].as_string()?,
        isin_code: row[2].as_string()?,
        indexer: row[3].as_string()?,
        deadline: row[4].as_string()?,
        quantity: row[5].as_f64()?,
        available_quantity: row[6].as_f64()?,
        unavailable_quantity: row[7].as_f64()?,
        reason: row[8].as_string()?,
        applied_value: row[9].as_f64()?,
        brute_value: row[10].as_f64()?,
        liquid_value: row[11].as_f64()?,
        updated_value: row[12].as_f64()?,
    };
    Some(treasury_direct)
}


pub fn parse_row(row: &[Data], sheet_name: &str) -> Option<InvestmentType> {
    match sheet_name {
        "Acoes" => match parse_stock(row) {
            Some(stock) => {
                Some(InvestmentType::Stock(stock))
            },
            None => None,
        },
        "Fundo de Investimento" => match parse_fii(row) {
            Some(fii) => {
                Some(InvestmentType::Fii(fii))
            },
            None => None,
        },
        "Tesouro Direto" => match parse_treasury_direct(row) {
            Some(treasury_direct) => {
                Some(InvestmentType::TreasuryDirect(treasury_direct))
            },
            None => None,
        },
        _ => None,
    }
}

pub fn parse_file(filepath: &str) -> Result<ParsedFile, XlsxError> {
    let mut workbook: Xlsx<_> = open_workbook(filepath).unwrap();

    let sheets = workbook.sheet_names().to_owned();

    let expected_sheets = vec![String::from("Acoes"), String::from("Fundo de Investimento"), String::from("Tesouro Direto")];
    assert!(sheets.iter().all(|item| expected_sheets.contains(&item)));


    let mut stock_rows: Vec<Stock> = Vec::new();
    let mut total_stock_value: f64 = 0.0;
    let mut fii_rows: Vec<Fii> = Vec::new();
    let mut total_fii_value: f64 = 0.0;
    let mut treasury_direct_rows: Vec<TreasuryDirect> = Vec::new();
    let mut total_treasury_direct_value: f64 = 0.0;


    for sheet_name in sheets {
        let sheet = workbook.worksheet_range(&sheet_name)?;
    
        let rows = sheet.rows();
        let _ = rows.clone().take(1).next().expect("No headers found");

        for row in rows.skip(1) {
            let empty_count = row.iter().filter(|item| item.is_empty()).count();

            let total_row = row.iter().filter(|item| {
                if let Some(item_str) = item.get_string() {
                    item_str == "Total"
                } else {
                    false
                }
            }).count();

            if empty_count > 0 || total_row == 1 {
                continue;
            }

            let total_values: Vec<f64> = row
                .iter()
                .filter_map(|item| item.as_f64())
                .collect();

            if total_values.len() == 1 {
                let total_value = total_values[0];
                match sheet_name.as_str() {
                    "Acoes" => {
                        total_stock_value = total_value;
                    },
                    "Fundo de Investimento" => {
                        total_fii_value = total_value;
                    },
                    "Tesouro Direto" => {
                        total_treasury_direct_value = total_value;
                    },
                    _ => (),
                }

            } else {
                let parsed_row = parse_row(row, &sheet_name).expect("Expected row to be parsed");

                match parsed_row {
                    InvestmentType::Stock(stock) => {
                        stock_rows.push(stock);
                    },
                    InvestmentType::Fii(fii) => {
                        fii_rows.push(fii);
                    },
                    InvestmentType::TreasuryDirect(treasury_direct) => {
                        treasury_direct_rows.push(treasury_direct);
                    }
                }
            }
        }
    }
    Ok(ParsedFile {
        stocks: stock_rows,
        total_value_stocks: total_stock_value,
        fiis: fii_rows,
        total_value_fiis: total_fii_value,
        treasury_directs: treasury_direct_rows,
        total_value_treasury_directs: total_treasury_direct_value,
    })
}
