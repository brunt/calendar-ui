extern crate actix_web;
extern crate actix;
#[macro_use]
extern crate rust_embed;
extern crate futures;

use actix_web::*;
use std::path::PathBuf;

#[derive(RustEmbed)]
#[folder = "public/"]
struct Asset;

//read events from (external) json file
//ui modal and /post call to add to json file
//add darksky api weather info to calendar

fn main() {
    let sys = actix::System::new("http-server");
    server::new(move || {
        App::new()
            .resource("/", |r| r.f(index))
            .resource("/{f:.*}", |r| r.f(dist))
    })
//            .bind("192.168.0.105:8000")
        .bind("127.0.0.1:8000")
        .expect("Cannot bind to port 8000")
        .start();

    println!("Started http server.");
    let _ = sys.run();
}

fn index(_: HttpRequest) -> Result <HttpResponse > {
    let html = Asset::get("index.html").unwrap();
    Ok(HttpResponse::Ok().body(html))
}

fn dist(req: HttpRequest) -> Result<HttpResponse> {
    let path: PathBuf = req.match_info().query("f")?;
    let file = Asset::get(&path.display().to_string()[..]).unwrap();
    Ok(HttpResponse::Ok().body(file))
}

