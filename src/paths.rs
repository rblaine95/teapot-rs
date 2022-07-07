use rocket::http::Status;
use rocket::response::{content, status};

// http://127.0.0.1:8000/418
#[catch(default)]
pub fn teapot() -> status::Custom<content::RawJson<&'static str>> {
    status::Custom(
        Status::ImATeapot,
        content::RawJson("{\"418\":\"I'm a teapot\"}"),
    )
}

// http://127.0.0.1:8000/404
#[catch(404)]
pub fn notfound() -> status::Custom<content::RawJson<&'static str>> {
    status::Custom(
        Status::NotFound,
        content::RawJson("{\"404\":\"Not Found\"}"),
    )
}

// http://127.0.0.1:8000/healthz
#[get("/")]
pub fn healthz() -> status::Custom<content::RawJson<&'static str>> {
    status::Custom(Status::Ok, content::RawJson("{\"status\":\"ok\"}"))
}
