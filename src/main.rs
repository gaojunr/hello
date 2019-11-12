use actix_web::{web, App, HttpRequest, HttpServer, Responder};

fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

fn main() {
    let i: i32 = 11;
    println!("{}", i);

    HttpServer::new(|| {
        App::new()
            .route(
                "/",
                web::get().to(|req: HttpRequest| {
                    let name = req.match_info().get("name").unwrap_or("World");
                    format!("Hello1 {}!", &name)
                }),
            )
            .route("/{name}", web::get().to(greet))
    })
    .bind("0.0.0.0:8080")
    .expect("Can not bind to port 8000")
    .run()
    .unwrap();
}
