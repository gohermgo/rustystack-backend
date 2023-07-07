// #![feature(proc_macro_hygiene, decl_macro)]

// const FRONTEND_DIR: &str = "../rustystack-ui";

// #[macro_use] extern crate rocket;

// use rocket::{
//     fs::NamedFile,
//     response::status::NotFound
// };
use actix_web::{web, App, HttpServer};
use rustystack_backend::run;
use std::path::PathBuf;

#[cfg(test)]
mod greet_tests {
    use actix_web::{body::MessageBody, test, web, HttpRequest, HttpResponse, Responder};
    use std::boxed::Box;

    #[tokio::test]
    async fn greet_parameterized() {
        let req = test::TestRequest::get()
            .uri("/greet/Steve")
            .param("name", "Steve")
            .to_http_request();
        let res = greet(req).await.body();
        assert_eq!(String::from("Hello Steve!").boxed(), res);
    }

    #[tokio::test]
    async fn greet_blank() {
        let req = test::TestRequest::get().uri("/greet").to_http_request();
        let res = greet(req).await;
        assert_eq!("Hello world!", res);
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run().await
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
