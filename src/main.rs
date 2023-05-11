use std::{error::Error, io, process};
use dotenv;
use sqlx::Row;
fn csv_parse() -> Vec<Set> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut data: Vec<Set> = vec![];
    for row in rdr.deserialize() {
        let result: Vec<String> = match row {
            Ok(res) => res,
            Err(err) => panic!("Couldnt deserialize: {:?}", err),
        };
        
        let _s = Set {
            number: result.get(0).unwrap().to_string(),
            set_name: result.get(1).unwrap().to_string(),
        };
        println!("{}, {}", _s.number, _s.set_name);

    }

    return data
}

struct Set {
    pub number: String,
    //pub theme: String,
    //pub sub_theme: String,
    //pub year: String,
    pub set_name: String,
    //pub minifigs: u32,
    //pub pieces: u32,
    //pub width: f32,
    //pub height: f32,
    //pub depth: f32,
    //pub launch_date: date,
    //pub exit_date: date,
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
    let url = dotenv::var("DATABASE_URL").unwrap();
    let pool = sqlx::postgres::PgPool::connect(&url).await.expect("something fucked");
        
    // create database migration
    sqlx::migrate!("./migrations").run(&pool).await?;

    //create(&set, &pool).await?;
    // csv_parse();
    Ok(())
}
