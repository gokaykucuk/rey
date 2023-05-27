use actix_web::{web, App, HttpResponse, HttpServer};
use num_cpus;
use serde::{Serialize, Deserialize};
use sled::Db;


// This is our data structure, a simple struct which will hold the id of a file.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct FileData {
    file_id: String,
}

// Implementation block for the FileData structure
impl FileData {
    // A constructor function for creating a new instance of FileData
    fn new(file_id: String) -> Self {
        FileData { file_id }
    }
}

// Asynchronous function to save a key-value pair into the sled database
async fn save(db: &Db, keys: Vec<String>, file_data: FileData) -> sled::Result<()> {
    // Serialize the keys and file data into byte arrays
    let key_bytes = bincode::serialize(&keys).unwrap();
    let value_bytes = bincode::serialize(&file_data).unwrap();
    // Insert the serialized key-value pair into the sled database
    db.insert(key_bytes, value_bytes)?;
    // Return Ok if everything went well
    Ok(())
}

// Asynchronous function to retrieve a value from the sled database given its key
async fn retrieve(db: &Db, keys: Vec<String>) -> sled::Result<Option<FileData>> {
    // Serialize the keys into byte array
    let key_bytes = bincode::serialize(&keys).unwrap();
    // Fetch the value associated with the key from the sled database
    match db.get(&key_bytes)? {
        Some(value_bytes) => {
            // If we have a value, deserialize it back into a FileData instance
            let file_data: FileData = bincode::deserialize(&value_bytes).unwrap();
            Ok(Some(file_data))
        },
        None => Ok(None),
    }
}


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
