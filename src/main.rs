#[macro_use]
extern crate rocket;
use rocket_prometheus::PrometheusMetrics;

mod paths;

#[cfg(test)]
mod tests;

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    let prometheus = PrometheusMetrics::new();

    rocket
        .attach(prometheus.clone())
        .register("/", catchers![paths::teapot])
        .register("/418", catchers![paths::teapot])
        .register("/404", catchers![paths::notfound])
        .mount("/healthz", routes![paths::healthz])
        .mount("/metrics", prometheus)
}
