use actix_web::{web, HttpResponse, Responder};
use bson::{doc, Document};
use mongodb::{Client};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use chrono::prelude::*;
use mysql::*;
use mysql::prelude::*;

const MONGO_DB: &'static str = "local";
const MONGO_COLL_LOGS: &'static str = "tweet";

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub nombre: String,
    pub comentario: String,
    pub fecha: String,
    pub hashtags: Vec<String>,
    pub upvotes: i64,
    pub downvotes: i64,
}

pub struct Tweet {
    pub nombre: String,
    pub comentario: String,
    pub fecha: String,
    pub hashtags: String,
    pub upvotes: i64,
    pub downvotes: i64,
    pub api: String,
}

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::post().to(add_log)),
    );
}

fn tweet_to_document(tweet: &Tweet) -> Document {
    let Tweet {
        nombre,
        comentario,
        fecha,
        hashtags,
        upvotes,
        downvotes,
        api,
    } = tweet;
    doc! {
        "nombre": nombre,
        "comentario": comentario,
        "fecha": fecha,
        "hashtags": hashtags,
        "upvotes": upvotes,
        "downvotes" : downvotes,
        "api":api,
    }
}

pub fn today(fecha: &String) -> NaiveDate {
    //let l = Local::today();

    let naive_date = NaiveDate::parse_from_str(&fecha, "%d/%m/%Y").unwrap();

    NaiveDate::from_ymd(naive_date.year(), naive_date.month(), naive_date.day())
}

async fn add_log(data: web::Data<Mutex<Client>>, user: web::Json<User>) -> impl Responder {
    let logs_collection = data
        .lock()
        .unwrap()
        .database(MONGO_DB)
        .collection(MONGO_COLL_LOGS);

        println!("entro al post");

        let mut hash_final =  String::from("");
        let mut contador = 0;
        let a="rust";
        for x in &user.hashtags {
            if contador == 0{
                hash_final = x.to_string();
            }
            else{
                hash_final = format!("{}, {}", hash_final, x);
            }
            contador = contador + 1;
        }

        let tweet = Tweet {
            nombre: user.nombre.clone(),
            comentario: user.comentario.clone(),
            fecha: user.fecha.clone(),
            hashtags: hash_final,
            upvotes: user.upvotes.clone(),
            downvotes: user.downvotes.clone(),
            api: a.to_string()
        };

        let url = "mysql://root:grupo30@35.188.131.97:3306/Proyecto1";
        let opts = Opts::from_url(&url.to_owned()).unwrap();
        let pool = Pool::new(opts).unwrap();
        let mut conn = pool.get_conn().unwrap();

        conn.exec_drop(
            "insert into tweet (nombre, comentario, fecha, hashtags, upvotes, downvotes, api)  
            values (:nombre, :comentario, :fecha, :hashtags, :upvotes, :downvotes, :api)",
            params! {
                "nombre" => &tweet.nombre,
                "comentario" => &tweet.comentario,
                "fecha" => today(&tweet.fecha),
                "hashtags" => tweet.hashtags.to_string(),
                "upvotes" => tweet.upvotes,
                "downvotes" => tweet.downvotes,
                "api" => "rust",
            },
        ).unwrap();

    match logs_collection.insert_one(tweet_to_document(&tweet), None).await {
        Ok(result) => HttpResponse::Ok().json(result.inserted_id),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
