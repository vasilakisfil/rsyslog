use rsyslog::{
    parser::{
        msg::{HerokuRouter, Raw},
        DateTime, SdParam, Skip, StructuredData,
    },
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

#[test]
fn complex_message2() {
    let msg: Result<Message, Error> = Message::parse(
        r#"<29>1 2016-02-21T04:32:57+00:00 web1 someservice - - [origin x-service="someservice"][meta sequenceId="14125553"] 127.0.0.1 - - 1456029177 "GET /v1/ok HTTP/1.1" 200 145 "-" "hacheck 0.9.0" 24306 127.0.0.1:40124 575"#,
    );

    assert_eq!(
        msg,
        Ok(rsyslog::Message {
            facility: 3,
            severity: 5,
            version: 1,
            timestamp: Some("2016-02-21T04:32:57+00:00"),
            hostname: Some("web1"),
            app_name: Some("someservice"),
            proc_id: None,
            msg_id: None,
            structured_data: vec![
                StructuredData {
                    id: "origin",
                    params: vec![SdParam {
                        name: "x-service",
                        value: "someservice"
                    },]
                },
                StructuredData {
                    id: "meta",
                    params: vec![SdParam {
                        name: "sequenceId",
                        value: "14125553"
                    },]
                }
            ],
            msg: Raw {
                msg: r#"127.0.0.1 - - 1456029177 "GET /v1/ok HTTP/1.1" 200 145 "-" "hacheck 0.9.0" 24306 127.0.0.1:40124 575"#
            }
        })
    );
}

#[test]
fn heroku_test_message() {
    let msg: Result<Message<_, Skip, HerokuRouter>, Error> = Message::parse("<158>1 2021-03-01T19:04:19.887695+00:00 host heroku router - at=info method=POST path=\"/api/v1/events/smartcam\" host=ratatoskr.mobility46.se request_id=5599e09a-f8e3-4ed9-8be8-6883ce842cf2 fwd=\"157.230.107.240\" dyno=web.1 connect=0ms service=97ms status=200 bytes=140 protocol=https");

    assert_eq!(
        msg,
        Ok(rsyslog::Message {
            facility: 19,
            severity: 6,
            version: 1,
            timestamp: Some("2021-03-01T19:04:19.887695+00:00"),
            hostname: Some("host"),
            app_name: Some("heroku"),
            proc_id: Some("router"),
            msg_id: None,
            structured_data: Skip,
            msg: HerokuRouter {
                at: "info",
                method: "POST",
                path: "/api/v1/events/smartcam",
                host: "ratatoskr.mobility46.se",
                request_id: "5599e09a-f8e3-4ed9-8be8-6883ce842cf2",
                fwd: "157.230.107.240",
                dyno: "web.1",
                connect: 0,
                service: 97,
                status: 200,
                bytes: 140,
                protocol: "https"
            }
        })
    );
}
