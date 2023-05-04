use actix_web::*;
use shamir_model::Update;

const SECRET_TOKEN_HEADER: &str = "X-Telegram-Bot-Api-Secret-Token";
const SECRET_TOKEN: &str = "";

async fn handle(update: web::Json<Update>) -> impl Responder {
    let update = update.0;
    println!("{update:?}");
    HttpResponse::Ok().body("Hello, World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().wrap(middleware::Logger::default()).service(
            web::resource("/")
                .guard(guard::Header(SECRET_TOKEN_HEADER, SECRET_TOKEN))
                .route(web::post().to(handle)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
