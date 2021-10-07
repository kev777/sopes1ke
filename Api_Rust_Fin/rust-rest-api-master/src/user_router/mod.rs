use crate::user_service::User;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use mysql::*;
use mysql::prelude::*;
use chrono::prelude::*;

#[get("/get-all-users")]
async fn get_all_users(app_data: web::Data<crate::AppState>) -> impl Responder {
    let action = app_data.service_manager.user.get();
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/get-user-email/{email}")]
async fn get_user_email(
    app_data: web::Data<crate::AppState>,
    email: web::Path<String>,
) -> impl Responder {
    let action = app_data.service_manager.user.get_user_email(&email);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/carga")]
async fn add_user(app_data: web::Data<crate::AppState>, user: web::Json<User>) -> impl Responder {
    let action = app_data.service_manager.user.create(&user);
    let result = web::block(move || action).await;
    let tweet = user.into_inner();

    let url = "mysql://root:grupo30@35.188.131.97:3306/Proyecto1";
    let opts = Opts::from_url(&url.to_owned()).unwrap();
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let mut hash_final =  String::from("");
    let mut contador = 0;
    for x in &tweet.hashtags {
        if contador == 0{
            hash_final = x.to_string();
        }
        else{
            hash_final = format!("{}, {}", hash_final, x);
        }
        contador = contador + 1;
    }

    conn.exec_drop(
        "insert into tweet (nombre, comentario, fecha, hashtags, upvotes, downvotes, api)  
        values (:nombre, :comentario, :fecha, :hashtags, :upvotes, :downvotes, :api)",
        params! {
            "nombre" => &tweet.nombre,
            "comentario" => &tweet.comentario,
            "fecha" => today(&tweet.fecha),
            "hashtags" => hash_final,
            "upvotes" => tweet.upvotes,
            "downvotes" => tweet.downvotes,
            "api" => "rust",
        },
    ).unwrap();


    match result {
        Ok(result) => HttpResponse::Ok().json(result.inserted_id),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/update-user")]
async fn update_user(
    app_data: web::Data<crate::AppState>,
    user: web::Json<User>,
) -> impl Responder {
    let action = app_data.service_manager.user.update(&user);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.modified_count),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[delete("/delete-user")]
async fn delete_user(
    app_data: web::Data<crate::AppState>,
    user: web::Json<User>,
) -> impl Responder {
    let action = app_data.service_manager.user.delete(&user.nombre);
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.deleted_count),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_users);
    cfg.service(get_user_email);
    cfg.service(add_user);
    cfg.service(update_user);
    cfg.service(delete_user);
}

pub fn today(fecha: &String) -> NaiveDate {
    let l = Local::today();

    let naive_date = NaiveDate::parse_from_str(&fecha, "%d/%m/%Y").unwrap();

    NaiveDate::from_ymd(naive_date.year(), naive_date.month(), naive_date.day())
}