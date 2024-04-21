use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::Value;

#[post("/mensaje")]
async fn recibir_mensaje(data: web::Json<Value>) -> impl Responder {
    // Obtener el texto del payload de la solicitud
    let text = data.get("text").and_then(Value::as_str).unwrap_or("");

    // Imprimir el mensaje recibido en la consola del servidor
    println!("Mensaje recibido en el servidor: {}", text);

    // Responder al cliente con un mensaje de confirmación
    "Mensaje recibido en el servidor!"
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("Pong")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Mensaje de inicio
    println!("Iniciando la API... server");

    // Crear y ejecutar el servidor
    HttpServer::new(|| {
        App::new()
            .service(recibir_mensaje)
            .service(ping)
    })
    .bind("0.0.0.0:8001")?
    .run()
    .await
}
