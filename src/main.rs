use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;
use std::fs;
use std::sync::Mutex;

// Estructura de datos para almacenar un contador
#[derive(Default)]
struct Counter {
    count: u32,
}

// Estructura de datos para almacenar el contenido del archivo de texto
#[derive(Default)]
struct FileContent {
    content: String,
}

// Estructura de datos para representar los datos de la calculadora
#[derive(Debug, Deserialize)]
struct CalculatorInput {
    num1: f64,
    num2: f64,
    operation: String,
}

// Estado compartido mutable para almacenar el contador y la calculadora
struct AppState {
    counter: Mutex<Counter>,
    file_content: Mutex<FileContent>,
}

// Manejador para la ruta raíz
async fn index(data: web::Data<AppState>) -> HttpResponse {
    let counter = data.counter.lock().unwrap(); // Bloqueo para obtener acceso exclusivo al contador
    let file_content = data.file_content.lock().unwrap(); // Bloqueo para obtener acceso exclusivo al contenido del archivo

    let html = format!(
        r#"
        <html>
        <head>
            <title>Contador, Contenido de Archivo y Calculadora</title>
        </head>
        <body>
            <h1>Contador</h1>
            <p>El contador actual es: {}</p>
            <form action="/incrementar" method="post">
                <button type="submit">Incrementar</button>
            </form>
            <h1>Contenido de Archivo</h1>
            <p>{}</p>
            <form action="/obtener_archivo" method="post">
                <button type="submit">Obtener Datos de Archivo</button>
            </form>
            <h1>Calculadora</h1>
            <form action="/calcular" method="post">
                <label for="num1">Numero 1:</label>
                <input type="text" id="num1" name="num1"><br><br>
                <label for="num2">Numero 2:</label>
                <input type="text" id="num2" name="num2"><br><br>
                <label for="operation">Operacion:</label>
                <select id="operation" name="operation">
                    <option value="add">Sumar</option>
                    <option value="subtract">Restar</option>
                    <option value="multiply">Multiplicar</option>
                    <option value="divide">Dividir</option>
                </select><br><br>
                <button type="submit">Calcular</button>
            </form>
        </body>
        </html>
    "#,
        counter.count, file_content.content
    );

    HttpResponse::Ok().content_type("text/html").body(html)
}

// Manejador para incrementar el contador
async fn increment(data: web::Data<AppState>) -> HttpResponse {
    let mut counter = data.counter.lock().unwrap(); // Bloqueo para obtener acceso exclusivo al contador
    counter.count += 1;

    HttpResponse::SeeOther().header("location", "/").finish()
}

// Manejador para obtener datos del archivo
async fn obtener_archivo(data: web::Data<AppState>) -> HttpResponse {
    let content = match fs::read_to_string("contenido.txt") {
        Ok(content) => content,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    println!("Contenido del archivo: {}", content);

    // let mut file_content = data.file_content.lock().unwrap();
    // file_content.content = content;

    HttpResponse::SeeOther().header("location", "/").finish()
}

// Manejador para calcular operaciones matemáticas
async fn calculate(input: web::Form<CalculatorInput>) -> HttpResponse {
    let result = match input.operation.as_str() {
        "add" => input.num1 + input.num2,
        "subtract" => input.num1 - input.num2,
        "multiply" => input.num1 * input.num2,
        "divide" => {
            if input.num2 != 0.0 {
                input.num1 / input.num2
            } else {
                return HttpResponse::BadRequest().body("Error: No se puede dividir por cero");
            }
        }
        _ => return HttpResponse::BadRequest().body("Operación no válida"),
    };

    let message = format!("El resultado de la operacion es: {}", result);
    HttpResponse::Ok().body(message)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppState {
        counter: Mutex::new(Counter::default()),
        file_content: Mutex::new(FileContent::default()),
    });

    // Iniciar el servidor Actix
    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone())
            .route("/", web::get().to(index))
            .route("/incrementar", web::post().to(increment))
            .route("/obtener_archivo", web::post().to(obtener_archivo))
            .route("/calcular", web::post().to(calculate))
    })
    .bind("127.0.0.1:8070")?
    .run()
    .await
}
