use rsyslog::{
    parser::{msg::LineRaw, StructuredData},
    Message,
};

type OneLineMessage<'a> = Message<'a, Option<&'a str>, Vec<StructuredData<'a>>, LineRaw<'a>>;

fn main() -> Result<(), String> {
    let msg = r#"<29>1 2016-02-21T04:32:57+00:00 web1 someservice - - - 127.0.0.1 - - 1456029177 "GET /v1/info HTTP/1.1" 200 145 "-" "hacheck 0.9.0" 24306 127.0.0.1:40124 575
<29>1 2016-02-21T05:32:57+00:00 web2 someservice - - - 127.0.0.1 - - 1456029177 "GET /v1/videos HTTP/1.1" 200 145 "-" "hacheck 0.9.0" 24306 127.0.0.1:40124 575
<29>1 2016-02-21T06:32:57+00:00 web3 someservice - - - 127.0.0.1 - - 1456029177 "GET /v1/users HTTP/1.1" 200 145 "-" "hacheck 0.9.0" 24306 127.0.0.1:40124 575"#;

    let mut messages: Vec<OneLineMessage> = Vec::new();

    let mut rem: &str = msg;
    while !rem.is_empty() {
        let tuple: (&str, OneLineMessage) =
            rsyslog::Message::parse_with_rem(rem).map_err(|e| e.to_string())?;
        rem = tuple.0;
        messages.push(tuple.1);
    }

    println!(
        "{:?}",
        messages.iter().map(|s| s.hostname).collect::<Vec<_>>()
    );

    Ok(())
}
