use rsyslog::{
    parser::{msg::Raw, DateTime, StructuredData},
    Error, Message,
};

#[test]
fn message_with_timestamp() {
    let msg: Result<Message<Option<DateTime>, Vec<StructuredData>, _>, Error> = Message::parse(
        "<1>1 2021-03-01T19:04:19.887695+00:00 host app_name proc_id msg_id - a message",
    );

    assert_eq!(
        msg,
        Ok(rsyslog::Message {
            facility: 0,
            severity: 1,
            version: 1,
            timestamp: Some(
                chrono::DateTime::parse_from_rfc3339("2021-03-01T19:04:19.887695+00:00").unwrap()
            ),
            hostname: Some("host"),
            app_name: Some("app_name"),
            proc_id: Some("proc_id"),
            msg_id: Some("msg_id"),
            structured_data: vec![],
            msg: Raw { msg: "a message" }
        })
    );
}
