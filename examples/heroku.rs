use rsyslog::{
    parser::{msg::HerokuRouter, Skip},
    Message,
};

type HerokuParser<'a> = Message<'a, Option<&'a str>, Skip, HerokuRouter<'a>>;

fn main() -> Result<(), String> {
    let msg = r#"<158>1 2021-03-01T19:04:19.887695+00:00 host heroku router - at=info method=POST path="/api/v1/events/smartcam" host=ratatoskr.mobility46.se request_id=5599e09a-f8e3-4ed9-8be8-6883ce842cf2 fwd="157.230.107.240" dyno=web.1 connect=0ms service=97ms status=200 bytes=140 protocol=https"#;
    let message = HerokuParser::parse(msg).map_err(|e| e.to_string())?;

    println!(
        "Received {} request on {} at {}",
        message.msg.method,
        message.msg.path,
        message.timestamp.ok_or("missing timestamp")?
    );

    Ok(())
}
