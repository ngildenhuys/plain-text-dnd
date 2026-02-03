use actix_web::http::header::ContentType;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct DnDCharacter {
    name: String,
    race: String,
    class: String,
    level: u32,
    hp: u32,
}

// Load character from YAML file
fn load_character(file_path: &str) -> Result<DnDCharacter, String> {
    let contents = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
    serde_yaml::from_str(&contents).map_err(|e| e.to_string())
}

// Save character to YAML file
fn save_character(file_path: &str, character: &DnDCharacter) -> Result<(), String> {
    let yaml = serde_yaml::to_string(&character).map_err(|e| e.to_string())?;
    fs::write(file_path, yaml).map_err(|e| e.to_string())?;
    Ok(())
}

// Serve the character edit page
#[get("/")]
async fn index() -> impl Responder {
    let file_path = "character.yaml"; // Path to the YAML file

    let character = match load_character(file_path) {
        Ok(c) => c,
        Err(_) => DnDCharacter {
            name: "Unknown".to_string(),
            race: "Unknown".to_string(),
            class: "Unknown".to_string(),
            level: 1,
            hp: 10,
        },
    };

    let html = include_str!("../static/index.html")
        .replace("{{ name }}", &character.name)
        .replace("{{ race }}", &character.race)
        .replace("{{ class }}", &character.class)
        .replace("{{ level }}", &character.level.to_string())
        .replace("{{ hp }}", &character.hp.to_string());

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html)
}

// Handle saving the updated character from form data
#[post("/save-character")]
async fn save_character_info(form: web::Form<DnDCharacter>) -> impl Responder {
    let file_path = "character.yaml"; // Path to the YAML file

    if let Err(err) = save_character(file_path, &form) {
        return HttpResponse::InternalServerError().body(format!("Error: {}", err));
    }

    HttpResponse::Ok().body("Character saved successfully!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index) // Serve the HTML form
            .service(save_character_info) // Handle form submissions
    })
    .bind("127.0.0.1:8090")?
    .run()
    .await
}
