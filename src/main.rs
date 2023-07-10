// #![feature(proc_macro_hygiene, decl_macro)]

// const FRONTEND_DIR: &str = "../rustystack-ui";

// #[macro_use] extern crate rocket;

// use rocket::{
//     fs::NamedFile,
//     response::status::NotFound
// };

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = std::net::TcpListener::bind(format!("{}:8000", rustystack_backend::PUBLIC_IP))?;
    rustystack_backend::run(listener)?.await
}

// Return the index file as a Rocket NamedFile
// async fn get_index() -> Result<NamedFile, NotFound<String>> {
//     NamedFile::open(format!("{}/dist/index.html", FRONTEND_DIR))
//         .await
//         .map_err(|e| NotFound(e.to_string()))
// }

// Create a route for any url that is a path from the /
// But I dont like the way done on adventuresofaliceandbob
// So i will probably think of something better later
// #[get("/<path..>")]
// async fn static_files(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
//     let path = PathBuf::from(format!("{}/dist/index.html", FRONTEND_DIR)).join(path);
//     match NamedFile::open(path).await {
//         Ok(f) => Ok(f),
//         Err(_) => get_index().await,
//     }
// }

// #[get("/")]
// async fn index() -> Result<NamedFile, NotFound<String>> {
//     get_index().await
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![index, static_files])
// }
