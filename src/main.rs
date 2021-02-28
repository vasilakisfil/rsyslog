mod parser;
mod helpers;

fn main() {
    let msg = r#"258 <158>1 2021-02-25T13:04:19.887695+00:00 host heroku router - at=info method=POST path="/api/v1/events/smartcam" host=ratatoskr.mobility46.se request_id=5599e09a-f8e3-4ed9-8be8-6883ce842cf2 fwd="157.230.107.240" dyno=web.1 connect=0ms service=97ms status=200 bytes=140 protocol=https"#;
    match parser::parser(msg) {
        Ok((_, msg)) => println!("{:?}", msg),
        Err(err) => match err {
            nom::Err::Error(e) => println!("{}", nom::error::convert_error(msg, e)),
            _ => println!("{}", err),
        },
    }
}
