mod ws;
mod db_layer;
use mysql::*;
use actix_web::{web, App, HttpServer};


#[actix_web::main]
async fn main() {
    let url = "mysql://root:grupo30@35.188.131.97:3306/Proyecto1";
    let opts = Opts::from_url(&url.to_owned()).unwrap();
    //let pool = Pool::new(opts).unwrap();
    //let mut conn = pool.get_conn().unwrap();

    let pool = match Pool::new(opts) {
        Ok(pool) => pool,
        Err(e) => {
            println!("Failed to open DB connection. {:?}", e); return;
        }
    };

    let shared_data = web::Data::new(pool);
 
    let server = match HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .service(ws::get_tweet)
            .service(ws::create_tweet)
    }).bind("0.0.0.0:3030") {
        Ok(s) => s,
        Err(e) => {
            println!("Failed to bind port. {:?}", e);
            return;
        }
    };
 
    match server.run().await {
        Ok(_) => println!("Server exited normally."),
        Err(e) => println!("Server exited with error: {:?}", e),
    };
    
}
    /*
    let utc: DateTime<Utc> = Utc::now(); 
     conn.exec_drop(
        "insert into tweet values (2, :nombre, :comentario, :fecha, :hash, :up, :do, :api)",
        params! {
            "nombre" => "P01",
            "comentario" => "P01_comentario",
            "hash" => "#prueba",
            "fecha" => today(),
            "up" => 30,
            "do" => 10,
            "api" => "rust"
        },
    ).unwrap();
    
    
}

fn today() -> NaiveDate {
    let l = Local::today();
 
    NaiveDate::from_ymd(l.year(), l.month(), l.day())
}
*/