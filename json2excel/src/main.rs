use clap::{App, Arg};
use serde_json::{Result, Value};
use xlsxwriter::{Workbook, Worksheet};

fn json_to_excel(json: &str, output_file: &str) -> Result<()> {
    // 解析 JSON 数据
    let data: Value = serde_json::from_str(json)?;

    // 创建 Excel 工作簿
    let mut workbook = Workbook::new(output_file)?;

    // 获取第一个工作表
    let mut worksheet = workbook.add_worksheet(None)?;

    // 遍历 JSON 数据
    let mut row = 0;
    for (key, value) in data.as_object().unwrap() {
        // 写入键
        worksheet.write_string(row, 0, key, None)?;

        // 写入值
        if let Some(array) = value.as_array() {
            for (col, item) in array.iter().enumerate() {
                worksheet.write_string(row, col + 1, &item.to_string(), None)?;
            }
        } else {
            worksheet.write_string(row, 1, &value.to_string(), None)?;
        }

        row += 1;
    }

    // 关闭工作簿
    workbook.close()?;

    Ok(())
}

fn main() {
    // 创建命令行解析器
    let matches = App::new("JSON to Excel Converter")
        .version("1.0")
        .author("Your Name")
        .about("Converts JSON to Excel")
        .arg(
            Arg::with_name("file-path")
                .short("f")
                .long("file-path")
                .value_name("FILE")
                .help("Specifies the JSON file path")
                .required(true),
        )
        .get_matches();

    // 获取命令行参数
    let file_path = matches.value_of("file-path").unwrap();

    // 读取 JSON 文件
    let json_data = std::fs::read_to_string(file_path)?;

    // 执行转换
    json_to_excel(&json_data, "output.xlsx").unwrap();

    println!("JSON 转换为 Excel 成功！");
}
