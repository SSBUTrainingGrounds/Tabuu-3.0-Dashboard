use rocket::{http::Status, Request};

#[catch(404)]
pub fn not_found(status: Status, req: &Request) -> String {
    format!(
        "{}: Could not find the requested path: '{}'",
        status,
        req.uri()
    )
}

#[catch(401)]
pub fn unauthorized(status: Status, req: &Request) -> String {
    let auth = req.headers().get_one("Authorization");

    if auth.is_none() {
        format!("{}: Missing Authorization Token", status)
    } else {
        format!("{}: Invalid Authorization Token", status)
    }
}

#[catch(403)]
pub fn forbidden(status: Status, _req: &Request) -> String {
    format!("{}", status)
}

#[catch(400)]
pub fn bad_request() -> &'static str {
    "Bad request!"
}

#[catch(default)]
pub fn default(status: Status, req: &Request) -> String {
    format!("{} ({})", status, req.uri())
}
