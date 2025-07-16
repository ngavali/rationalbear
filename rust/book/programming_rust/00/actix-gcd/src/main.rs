use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    m: u64,
    n: u64,
}

fn gcd(mut x: u64, mut y: u64) -> u64 {
    assert!(x != 0 && y != 0);
    while x != 0 {
        if x < y {
            let t = x;
            x = y;
            y = t;
        }
        x = x % y
    }
    y
}

fn main() {}

/*
 * #actix-web = "1.0.8"
 * #serde = { version = "1.0", features = ["derive"] }

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Serving on http://localhost:3000...");
    server
        .bind("0.0.0.0:3000")
        .expect("Error binding server to address")
        .run()
        .expect("Error running server");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                    <input type="text" name="m" />
                    <input type="text" name="n" />
                    <button type="submit">Compute GCD</button>
                </form>
            "#,
    )
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.m == 0 || form.n == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let reponse = format!(
        "The greatest common divisor for the numbers {} and {} 
    is <b>{}</b>\n",
        form.m,
        form.n,
        gcd(form.m, form.n)
    );

    HttpResponse::Ok().content_type("text/html").body(reponse)
}
*/
