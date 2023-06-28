#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::{
    fs::NamedFile,
    response::status::NotFound
};
use std::path::PathBuf;

// Return the index file as a Rocket NamedFile
async fn get_index() -> Result<NamedFile, NotFound<String>> {
    NamedFile::open("../rustystack-ui/dist/index.html")
        .await
        .map_err(|e| NotFound(e.to_string()))
}

// Create a route for any url that is a path from the /
// But I dont like the way done on adventuresofaliceandbob
// So i will probably think of something better later
#[get("/<path..>")]
async fn static_files(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("../rustystack-ui/dist").join(path);
    match NamedFile::open(path).await {
        Ok(f) => Ok(f),
        Err(_) => get_index().await,
    }
}

#[get("/")]
async fn index() -> Result<NamedFile, NotFound<String>> {
    get_index().await
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, static_files])
}
