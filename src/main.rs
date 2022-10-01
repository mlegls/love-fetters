use actix_files as fs;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use sailfish::TemplateOnce;

struct Letter{
    date: String,
    author: String,
    link: String,
}

#[derive(TemplateOnce)]
#[template(path = "index.html")]
struct IndexTemplate {
    letters: Vec<Letter>,
}

#[derive(TemplateOnce)]
#[template(path = "letter.html")]
struct LetterTemplate {
    info: Letter,
    content: String,
}

#[get("/")]
async fn index() -> impl Responder {
    let paths = std::fs::read_dir("./letters").unwrap();
    let ctx = IndexTemplate {
        letters: paths.filter_map(|file| {
            file.ok().and_then(|f| 
                f.path().file_name()
                .and_then(|n| n.to_str().map(|s| {
                    let [date, author, _]: [&str; 3] = s.split("_").collect::<Vec<&str>>().try_into().unwrap();
                    Letter {
                        date: date.to_string(),
                        author: author.to_string(),
                        link: s.to_string(),
                    }
                }
                )))
        }).collect::<Vec<Letter>>(),
    };
    HttpResponse::Ok().body(ctx.render_once().unwrap())
}

#[get("/letter/{letter}")]
async fn letter(letter: web::Path<String>) -> impl Responder {
    let file_content = std::fs::read_to_string(format!("./letters/{}", letter)).unwrap();
    let parser = pulldown_cmark::Parser::new(&file_content);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    let [date, author, _]: [&str; 3] = letter.split("_").collect::<Vec<&str>>().try_into().unwrap();
    let ctx = LetterTemplate {
        info: Letter {
            date: date.to_string(),
            author: author.to_string(),
            link: letter.to_string(),
        },
        content: html_output,
    };
    HttpResponse::Ok().body(ctx.render_once().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT")
    .unwrap_or_else(|_| "8000".to_string())
    .parse()
    .expect("PORT must be a number");

    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", "./public").show_files_listing())
            .service(index)
            .service(letter)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
