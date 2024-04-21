use hyper::{Body, Client, Request, Response, Uri};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use hyper::Server;
use std::net::SocketAddr;
use hyper::client::HttpConnector;

use hyper::Error as HyperError;

async fn enviar_a_otro_servidor(parametros: &str) -> Result<hyper::Response<hyper::Body>, HyperError> {
    // Crear un cliente HTTP
    let http = HttpConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(http);

    // Crear la URL del otro servidor
    let uri = Uri::from_static("http://rust-server:8001/mensaje");

    // Crear una solicitud POST con los parámetros recibidos en el cuerpo
    let request = Request::post(uri)
        .header("Content-Type", "application/json")
        .body(Body::from(parametros.to_string()))
        .expect("Failed to build request");

    // Enviar la solicitud al otro servidor y devolver su respuesta
    let response = client.request(request).await?;
    Ok(response)
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        // Serve some instructions at /
        (&hyper::Method::GET, "/") => Ok(Response::new(Body::from(
            "Try POSTing data to /echo such as: `curl localhost:8080/echo -XPOST -d 'hello world'`",
        ))),

        // Simply echo the body back to the client.
        (&hyper::Method::POST, "/echo") => Ok(Response::new(req.into_body())),

        (&hyper::Method::POST, "/echo/reversed") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;

            let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
            Ok(Response::new(Body::from(reversed_body)))
        }

        (&hyper::Method::POST, "/server") => {
            // Obtener los parámetros del cuerpo de la solicitud
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            let parametros = String::from_utf8_lossy(&whole_body).to_string();
        
            // Enviar los parámetros al otro servidor
            let response = enviar_a_otro_servidor(&parametros).await?;
        
            // Devolver la respuesta del otro servidor al cliente original
            Ok(response)
        }

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = hyper::StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000)); // Escuchar en el puerto 8000
    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(handle_request)) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Error en el servidor: {}", e);
    }

    Ok(())
}
