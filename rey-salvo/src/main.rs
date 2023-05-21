use salvo::prelude::*;
use tracing_subscriber;
use redis::{Client, Commands,RedisResult};

fn put_message_to_redis(key: &str, message: &str) -> RedisResult<()> {
    let client = Client::open("redis://dragonfly/")?;
    let mut con = client.get_connection()?;
    let _: () = con.set(key, message)?;
    Ok(())
}

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

#[handler]
async fn put_message(req: &mut Request, res: &mut Response) {
    // Read the POST parameter named "data" into a string.
    let message = req.form::<String>("message").await.unwrap();
    put_message_to_redis("my_key", &message).unwrap();
    res.render(message);
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new().get(hello).push(
        Router::with_path("put").post(put_message)
    );
    
    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}