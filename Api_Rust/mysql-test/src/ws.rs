use super::db_layer::*;
use actix_web::*;
use mysql::*;


#[get("/tweet/{id}/")]
async fn get_tweet(path: web::Path<u64>, data: web::Data<Pool>) -> HttpResponse {
    let tweet_id = *path;
 
    match data
        .get_conn()
        .and_then(|mut conn| find_tweet_by_id(&mut conn, tweet_id))
    {
        Ok(res) => match res {
            Some(tweet) => HttpResponse::Ok().json(tweet),
            None => HttpResponse::NotFound().finish(),
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/tweet/")]
async fn create_tweet(tweet_json: web::Json<Tweet>, data: web::Data<Pool>) -> HttpResponse {
    let tweet = tweet_json.into_inner();
 
    match data
        .get_conn()
        .and_then(|mut conn| insert_tweet(&mut conn, &tweet))
    {
        Ok(tweet_id) => {
            //Return a Product with id set.
            HttpResponse::Ok().json(Tweet {
                id: tweet_id,
                ..tweet
            })
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("Something went wrong: {}", e)),
    }
}


