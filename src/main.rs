use std::{error::Error, io, process};

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
    pub set_name: String,
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
    //if let Err(err) = example() {
    //    println!("error running example: {}", err);
    //    process::exit(1);
    //}
    // create connection pool
    // let url = "";
    // let pool = sqlx::postgres::PgPool::connect(url).await?;
        
    // create database migration
    // sqlx::migrate!("./migrations").run(&pool).await?;

    // let set = Set {
        // number: "10123-1".to_string(),
        // set_name: "Cloud City".to_string(),
    // };

    // create(&set, &pool).await?;
    csv_parse();
    Ok(())
}
