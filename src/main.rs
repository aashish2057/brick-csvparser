use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Set {
    pub number: String,
    pub theme: Option<String>,
    pub sub_theme: Option<String>,
    pub year: Option<String>,
    pub set_name: String,
    pub minifigs: Option<i32>,
    pub pieces: Option<i32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub depth: Option<f32>,
    pub launch_date: Option<String>,
    pub exit_date: Option<String>, 
}

fn csv_parse(data: &mut Vec<Set>) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("_sets.csv")?;
    for row in rdr.deserialize::<Set>() {
        data.push(row?);
    }
    Ok(())
}

async fn create(set: &Set, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO sets (number, theme, sub_theme, year, set_name, minifigs, pieces, width, height, depth, launch_date, exit_date) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)";

    sqlx::query(query)
        .bind(&set.number)
        .bind(&set.theme)
        .bind(&set.sub_theme)
        .bind(&set.year)
        .bind(&set.set_name)
        .bind(set.minifigs)
        .bind(set.pieces)
        .bind(set.width)
        .bind(set.height)
        .bind(set.depth)
        .bind(&set.launch_date)
        .bind(&set.exit_date)
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
    //sqlx::migrate!("./migrations").run(&pool).await?;
    let mut data: Vec<Set> = vec![];

    // parse csv into vector
    match csv_parse(&mut data) {
        Ok(()) => println!("data parsed"), 
        Err(err) => {
            panic!("Read csv failed: {:?}", err);
        }
    };

    for s in data.iter() {
        create(&s, &pool).await?;
    }
    Ok(())
}
