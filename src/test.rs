use actix_web::{get, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use actix_cors::Cors;
use actix_web::http::header;

// #[macro_use]
// extern crate derive_new;

#[derive(Serialize, Deserialize)]
struct MyObj {
    content: String,
}

impl MyObj {
    fn new(name_value: String) -> MyObj {
        MyObj { content: name_value }
    }
}

// #[derive(new)]1
// struct Book {
//     title: String,
//     author: String,
//     genres: String,
//     pages: u32,
// }


// #[test]
// fn test_book() {
//     let common_book = Book::new(
//         String::from("Common"),
//         String::new(),
//         String::from("Programming"),
//         100,
//     );
//     assert_eq!(
//         common_book,
//         Book {
//             title: String::from("Common"),
//             author: String::new(),
//             genres: "Programming".to_string(),
//             pages: 100
//         }
//     );

//     let another_book = Book::new(String::from("Hey"), String::from("abc"), "Fiction".to_string(), 150);
//     let zz = MyObj::new(String::from("Hey"));
// }


#[get("/items/{content}")]
async fn index(_obj: web::Path<MyObj>) -> Result<HttpResponse> {

    // let a = MyObj {
    //     name: obj.name.to_string(),
    // };

    let mut v: Vec<MyObj> = Vec::new();

    let qq = MyObj::new(String::from("Hey"));
    let pp = MyObj::new(String::from("xxxx"));
    v.push(qq);
    v.push(pp);

    //  let a = MyObj {
    //     name: obj.name.to_string(),
    // };
    Ok(HttpResponse::Ok().json(v))
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{App, HttpServer};

//     HttpServer::new(|| App::new().wrap(
//         Cors::default()
//     // .allowed_origin("http://localhost:3000")
//     .allowed_methods(vec!["GET", "POST"])
//     .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
//     .allowed_header(header::CONTENT_TYPE)
//     .max_age(3600)
//       ).service(index))
//         .bind("127.0.0.1:8080")?
//         .run()
//         .await
// }


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     use actix_web::{App, HttpServer};
//     HttpServer::new(|| {
//         App::new()
//             .service(index)
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }