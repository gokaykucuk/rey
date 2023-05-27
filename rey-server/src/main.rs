use actix_web::{web, App, HttpResponse, HttpServer};
use num_cpus;
// caruse redis::{RedisResult, Client, Commands};

// fn put_message_to_redis(key: &str, message: &str) -> RedisResult<()> {
//     let client = Client::open("redis://dragonfly/")?;
//     let mut con = client.get_connection()?;
//     let _: () = con.set(key, message)?;
//     Ok(())
// }

async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello World")
}

async fn put_message(message: web::Form<String>) -> HttpResponse {
    // put_message_to_redis("my_key", &message).unwrap();
    HttpResponse::Ok().body(message.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    print!("Starting server");
    let num_workers = num_cpus::get();
    print!("Number of workers: {}", num_workers);
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/put", web::post().to(put_message))
    })
    .workers(num_workers);
    print!("\n\n======");
    print!("\n\nStarting server on port 5800\n\n");
    
    server.bind("0.0.0.0:5800")?.run().await
}
