#[cfg(test)]
use rocket::{http::Status, local::blocking::Client};

use super::rocket;

#[test]
fn test_teapot() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let req = client.get("/418");
	let resp = req.dispatch();
    assert_eq!(resp.status(), Status::ImATeapot);
}

#[test]
fn test_notfound() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let req = client.get("/404");
	let resp = req.dispatch();
    assert_eq!(resp.status(), Status::NotFound);
}

#[test]
fn test_healthz() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let req = client.get("/healthz");
	let resp = req.dispatch();
    assert_eq!(resp.status(), Status::Ok);
}

#[test]
fn test_metrics() {
	let client = Client::tracked(rocket()).expect("valid rocket instance");
	let req = client.get("/metrics");
	let resp = req.dispatch();
	assert_eq!(resp.status(), Status::Ok);
	assert_eq!(resp.content_type(), Some(rocket::http::ContentType::Plain));
}
