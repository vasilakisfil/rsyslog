use rsyslog::Message;

fn main() -> Result<(), String> {
    let msg = r#"<29>1 2016-02-21T04:32:57+00:00 web1 someservice - - [origin x-service="someservice"][meta sequenceId="14125553"] 127.0.0.1 - - 1456029177 "GET /v1/ok HTTP/1.1" 200 145 "-" "hacheck 0.9.0" 24306 127.0.0.1:40124 575"#;
    //default Message basically is: Message<'a, Option<&'a str>, Vec<StructuredData>, Raw<'a>>
    let message: Message = rsyslog::Message::parse(msg).map_err(|e| e.to_string())?;

    println!(
        "{}",
        message
            .structured_data
            .first()
            .ok_or("missing structured data")?
            .id
    );

    Ok(())
}
