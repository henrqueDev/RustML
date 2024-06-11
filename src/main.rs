use std::fs::File;

use chrono::prelude::*;
use polars::prelude::*;

fn main() {
    let mut df: DataFrame = df!(
        "integer" => &[1, 2, 3],
        "date" => &[
                NaiveDate::from_ymd_opt(2025, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                NaiveDate::from_ymd_opt(2025, 1, 2).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                NaiveDate::from_ymd_opt(2025, 1, 3).unwrap().and_hms_opt(0, 0, 0).unwrap(),
        ],
        "float" => &[4.0, 5.0, 6.0],
        "string" => &["a", "b", "c"],
    ).unwrap();

    println!("{} \n\n\n ---------- \n\n", df);

    let mut file = File::create("output.csv").expect("could not create file");
    let _ = CsvWriter::new(&mut file)
        .include_header(true)
        .with_separator(b',')
        .finish(&mut df);
    let df_csv = CsvReadOptions::default()
        .with_infer_schema_length(None)
        .with_has_header(true)
        .try_into_reader_with_file_path(Some("output.csv".into())).unwrap().finish().unwrap();
    //let df_final: DataFrame = df!("mijo" => df_csv);
    println!("{:#?}", df_csv);


    let df = CsvReadOptions::default()
    .try_into_reader_with_file_path(Some("Iris.csv".into()))
    .unwrap()
    .finish()
    .unwrap();
    let mask = df.column("SepalLengthCm").unwrap().f64().unwrap().gt(5.0);
    let df_small = df.filter(&mask);
    #[allow(deprecated)]
    let df_agg = df_small.expect("Deu ruim")
        .group_by(["Species"])
        .expect("Ruim")
        .select(["SepalWidthCm"])
        .mean()
        .unwrap();
    println!("{:#?}", df_agg);

}