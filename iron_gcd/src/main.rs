//
// Based on an example from “Programming Rust” 1st Edition, by Jim Blandy, Jason Orendorff, and Leonora Tindall (MIT License).
// https://github.com/ProgrammingRust/examples/blob/2927097382be5aad0b3737c2a70d544a8a030f2e/iron-gcd/src/main.rs
//

extern crate iron;
extern crate router;
extern crate urlencoded;
#[macro_use] extern crate mime;

// prelude modules contain common elements and it's allowed to load all of them by *
use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("Access to server: http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}

fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    // r# means raw array of characters and number of # has to be the same at the top and bottom
    response.set_mut(r#"
    <title>GCD Calculator</title>
    <form action="/gcd" method="post">
        <input type="text" name="n" />
        <input type="text" name="n" />
        <button type="submit">Calculate GCD</button>
    </form>
    "#);

    // Result type, so it's Ok(r) or Err(e)
    Ok(response)

}

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Parsing error: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map
    };

    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("No arguments 'n'\n"));
            return Ok(response);
        }
        Some(nums) => nums
    };

    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(
                    format!("Value of n is not a number: {:?}\n",
                unparsed));
                return Ok(response);
            }
            Ok(n) => { numbers.push(n);}
        }
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    // r# means raw array of characters and number of # has to be the same at the top and bottom
    response.set_mut(
        format!("Biggest common dividor of {:?} is <b>{}</b>\n",
    numbers, d)
    );

    Ok(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    // In case of 0 there will be panic: assertion failed: n != 0 && m != 0
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;

        }
        m = m % n;
    }
    n
}

#[test] // attribute, f.e. #[test], should be above function
fn test_gcd(){
    assert_eq!(gcd(14,15), 1);
    assert_eq!(gcd(10,30), 10);
    assert_eq!(gcd(6,18), 6);
}