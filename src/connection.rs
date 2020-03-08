use clickhouse_rs::{Pool, types::SqlType};
use tokio::prelude::*;

fn read(address: String, query: String) {
    let pool = Pool::new(address.as_str());
    let done = pool
        .get_handle()
        .and_then(|c| c.ping())
        .and_then(move |c| c.query(&query).fetch_all())
        .and_then(move |(_, block)| {
            for row in block.rows() {
                for i in 0..row.len() {
                    match row.sql_type(i) {
                        Ok(SqlType::UInt8) => {
                            let val: u8 = row.get(i)?;
                            println!("{}", val);
                        },
                        Ok(SqlType::UInt16) => {
                            let val: u16 = row.get(i)?;
                            println!("{}", val);
                        },
                        Ok(SqlType::UInt32) => {
                            let val: u32 = row.get(i)?;
                            println!("{}", val);
                        },
                        Ok(SqlType::UInt64) => {
                            let val: u64 = row.get(i)?;
                            println!("{}", val);
                        },
                        Ok(SqlType::Int8) => {
                            let val: i8 = row.get(i)?;
                            println!("{}", val);
                        },
                        Ok(SqlType::Int16) => {
                            let val: i16 = row.get(i)?;
                            println!("{}", val);
                        },
                        Ok(SqlType::Int32) => {
                            let val: i32 = row.get(i)?;
                            println!("{}", val);
                        },
                        Ok(SqlType::Int64) => {
                            let val: i64 = row.get(i)?;
                            println!("{}", val);
                        },
                        Ok(SqlType::String) => {
                            let val: &str = row.get(i)?;
                            println!("{}", val);
                        },
                        Ok(SqlType::FixedString(_)) => {
                            let val: &str = row.get(i)?;
                            println!("{}", val);
                        },
                        Ok(SqlType::Float32) => {
                            let val: f32 = row.get(i)?;
                            println!("{}", val);
                        },
                        Ok(SqlType::Float64) => {
                            let val: f64 = row.get(i)?;
                            println!("{}", val);
                        },
                        Ok(SqlType::Nullable(t)) => {
                            match t {
                                SqlType::UInt8 => {
                                    let val: Option<u8> = row.get(i)?;
                                    println!("{:?}", val);
                                },
                                SqlType::UInt16 => {
                                    let val: Option<u16> = row.get(i)?;
                                    println!("{:?}", val);
                                },
                                SqlType::UInt32 => {
                                    let val: Option<u32> = row.get(i)?;
                                    println!("{:?}", val);
                                },
                                SqlType::UInt64 => {
                                    let val: Option<u64> = row.get(i)?;
                                    println!("{:?}", val);
                                },
                                SqlType::Int8 => {
                                    let val: Option<i8> = row.get(i)?;
                                    println!("{:?}", val);
                                },
                                SqlType::Int16 => {
                                    let val: Option<i16> = row.get(i)?;
                                    println!("{:?}", val);
                                },
                                SqlType::Int32 => {
                                    let val: Option<i32> = row.get(i)?;
                                    println!("{:?}", val);
                                },
                                SqlType::Int64 => {
                                    let val: Option<i64> = row.get(i)?;
                                    println!("{:?}", val);
                                },
                                SqlType::String => {
                                    let val: Option<&str> = row.get(i)?;
                                    println!("{:?}", val);
                                },
                                SqlType::FixedString(_) => {
                                    let val: Option<&str> = row.get(i)?;
                                    println!("{:?}", val);
                                },
                                SqlType::Float32 => {
                                    let val: Option<f32> = row.get(i)?;
                                    println!("{:?}", val);
                                },
                                SqlType::Float64 => {
                                    let val: Option<f64> = row.get(i)?;
                                    println!("{:?}", val);
                                },
                                _ => panic!(),
                            }
                        },
                        _ => panic!(),
                    };
                }
            }
            Ok(())
        })
        .map_err(|err| println!("database error: {}", err));

    tokio::run(done)
}
