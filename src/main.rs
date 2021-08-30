use rocket::http::Status;
use rocket::response::Redirect;
use rocket::route::{Handler, Outcome};

/// A default catch-all handler that redirects to the trailing-slash version
/// of the URL, if the URL doesn't already end in a trailing slash.
#[derive(Clone, Debug)]
struct AppendSlashes;

#[rocket::async_trait]
impl Handler for AppendSlashes {
    async fn handle<'r>(&self, req: &'r rocket::Request<'_>, data: rocket::Data<'r>) -> Outcome<'r> {
        let has_trailing_slash = req.uri().path().ends_with('/');
        if !has_trailing_slash {
            match req.uri().map_path(|p| format!("{}/", p)) {
                Some(fixed_path) => Outcome::from(req, Redirect::to(fixed_path.to_owned())),
                None => Outcome::failure(Status::InternalServerError),
            }
        } else {
            Outcome::forward(data)
        }
    }
}

fn main() {
    println!("Hello, world!");
}
