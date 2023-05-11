use std::{error::Error, io, process};
use dotenv;
use serde::Deserialize;
use sqlx::Row;

#[derive(Debug, Deserialize)]
struct Set {
    pub number: String,
    pub theme: Option<String>,
    pub sub_theme: Option<String>,
    pub year: Option<String>,
    pub set_name: String,
    pub minifigs: Option<u32>,
    pub pieces: Option<u32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub depth: Option<f32>,
    //pub launch_date: date,
    //pub exit_date: date,
}

fn csv_parse(data:&mut Vec<Set>) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("_sets.csv")?;
    for row in rdr.deserialize::<Set>() {
        data.push(row?);
    }
    Ok(()) 
}


async fn create(set: &Set, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO sets (number, set_name) VALUES ($1, $2)";

    sqlx::query(query)
        .bind(&set.number)
        .bind(&set.set_name)
        .execute(pool)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // create connection pool
    // let url = dotenv::var("DATABASE_URL").unwrap();
    // let pool = sqlx::postgres::PgPool::connect(&url).await.expect("something fucked");
        
    // create database migration
    // sqlx::migrate!("./migrations").run(&pool).await?;
    let mut data: Vec<Set> = vec![];

    // parse csv into vector
    match csv_parse(&mut data){
        Ok(()) => {
            println!("{:?}", data);
        },
        Err(err) => {
            panic!("Read csv failed: {:?}", err);
        },
    };
    Ok(())
}
