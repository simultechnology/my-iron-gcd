extern crate iron;
extern crate mime;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::intrinsics::rotate_left;

fn main() {
    println!("Serving on http://localhost:3000...");
    Iron::new(get_form).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    router.get("/", get_form, "root");

    response.set_mut(status::Ok);
    //response.headers.set(mime::TEXT_HTML_UTF_8);
    response.headers.set(iron::headers::ContentType("text/html; charset=utf-8".parse::<iron::mime::Mime>().unwrap()));
    response.set_mut(r#"
    <title>GCD Calculator</title>
    <form action="/gcd" method="post">
        <input type="text" name="n"/>
        <input type="text" name="n"/>
        <button type="submit">Compute GCD</button>
    </form>
    "#);

    Ok(response)
}
