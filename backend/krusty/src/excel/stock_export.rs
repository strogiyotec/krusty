use rocket::serde::json::Json;
use xlsxwriter::*;

pub fn generate_report(
    stocks: Json<
        Vec<crate::models::stocks::Stock>,
    >,
) -> Result<(), XlsxError> {
    let workbook = Workbook::new("report.xlsx");
    let mut sheet = workbook
        .add_worksheet(Some("Global report"))?;
    //header
    write_header(&workbook, &mut sheet)?;
    let mut index = 1;
    for stock in stocks.iter() {
        sheet.write_string(
            index,
            0,
            stock.ticker,
            None,
        )?;
        sheet.write_number(
            index,
            1,
            stock.amount as f64,
            None,
        )?;
        index += 1;
    }
    return Ok(());
}

fn write_header(
    workbook: &Workbook,
    sheet: &mut Worksheet,
) -> Result<(), XlsxError> {
    let header_format = header_format(&workbook);
    sheet.write_string(
        0,
        0,
        "Ticker",
        Some(&header_format),
    )?;
    return sheet.write_string(
        0,
        1,
        "Quantity",
        Some(&header_format),
    );
}

fn header_format(workbook: &Workbook) -> Format {
    return workbook
        .add_format()
        .set_bg_color(FormatColor::Green)
        .set_align(
            FormatAlignment::CenterAcross,
        );
}
