use rsyslog::{
    parser::{
        msg::{HerokuRouter, Raw},
        SdParam, Skip, StructuredData,
    },
    Error, Message,
};

#[test]
fn empty_message() {
    let msg = "<1>1 - - - - - -";
    let msg: Result<Message, Error> = Message::parse(msg);

    assert_eq!(
        msg,
        Ok(rsyslog::Message {
            facility: 0,
            severity: 1,
            version: 1,
            timestamp: None,
            hostname: None,
            app_name: None,
            proc_id: None,
            msg_id: None,
            structured_data: vec![],
            msg: Raw { msg: "" }
        })
    );
}

#[test]
fn timestamp_message() {
    let msg = "<1>1 2021-03-01T19:04:19.887695+00:00 - - - - -";
    let msg: Result<Message, Error> = Message::parse(msg);

    assert_eq!(
        msg,
        Ok(rsyslog::Message {
            facility: 0,
            severity: 1,
            version: 1,
            timestamp: Some("2021-03-01T19:04:19.887695+00:00"),
            hostname: None,
            app_name: None,
            proc_id: None,
            msg_id: None,
            structured_data: vec![],
            msg: Raw { msg: "" }
        })
    );
}

#[test]
fn timestamp_host_message() {
    let msg = "<1>1 2021-03-01T19:04:19.887695+00:00 host - - - -";
    let msg: Result<Message, Error> = Message::parse(msg);

    assert_eq!(
        msg,
        Ok(rsyslog::Message {
            facility: 0,
            severity: 1,
            version: 1,
            timestamp: Some("2021-03-01T19:04:19.887695+00:00"),
            hostname: Some("host"),
            app_name: None,
            proc_id: None,
            msg_id: None,
            structured_data: vec![],
            msg: Raw { msg: "" }
        })
    );
}

#[test]
fn timestamp_host_app_name_message() {
    let msg = "<1>1 2021-03-01T19:04:19.887695+00:00 host app_name - - -";
    let msg: Result<Message, Error> = Message::parse(msg);

    assert_eq!(
        msg,
        Ok(rsyslog::Message {
            facility: 0,
            severity: 1,
            version: 1,
            timestamp: Some("2021-03-01T19:04:19.887695+00:00"),
            hostname: Some("host"),
            app_name: Some("app_name"),
            proc_id: None,
            msg_id: None,
            structured_data: vec![],
            msg: Raw { msg: "" }
        })
    );
}

#[test]
fn timestamp_host_app_name_proc_id_message() {
    let msg = "<1>1 2021-03-01T19:04:19.887695+00:00 host app_name proc_id - -";
    let msg: Result<Message, Error> = Message::parse(msg);

    assert_eq!(
        msg,
        Ok(rsyslog::Message {
            facility: 0,
            severity: 1,
            version: 1,
            timestamp: Some("2021-03-01T19:04:19.887695+00:00"),
            hostname: Some("host"),
            app_name: Some("app_name"),
            proc_id: Some("proc_id"),
            msg_id: None,
            structured_data: vec![],
            msg: Raw { msg: "" }
        })
    );
}

#[test]
fn timestamp_host_app_name_proc_id_msg_id_message() {
    let msg = "<1>1 2021-03-01T19:04:19.887695+00:00 host app_name proc_id msg_id -";
    let msg: Result<Message, Error> = Message::parse(msg);

    assert_eq!(
        msg,
        Ok(rsyslog::Message {
            facility: 0,
            severity: 1,
            version: 1,
            timestamp: Some("2021-03-01T19:04:19.887695+00:00"),
            hostname: Some("host"),
            app_name: Some("app_name"),
            proc_id: Some("proc_id"),
            msg_id: Some("msg_id"),
            structured_data: vec![],
            msg: Raw { msg: "" }
        })
    );
}

#[test]
fn timestamp_host_app_name_proc_id_structured_data_message() {
    let msg = concat!(
        "<1>1 2021-03-01T19:04:19.887695+00:00 ",
        "host app_name proc_id msg_id [structured_data]"
    );
    let msg: Result<Message, Error> = Message::parse(msg);

    assert_eq!(
        msg,
        Ok(rsyslog::Message {
            facility: 0,
            severity: 1,
            version: 1,
            timestamp: Some("2021-03-01T19:04:19.887695+00:00"),
            hostname: Some("host"),
            app_name: Some("app_name"),
            proc_id: Some("proc_id"),
            msg_id: Some("msg_id"),
            structured_data: vec![StructuredData {
                id: "structured_data",
                params: vec![]
            }],
            msg: Raw { msg: "" }
        })
    );
}

#[test]
fn complex_message() {
    let msg = concat!(
        "<1>1 2021-03-01T19:04:19.887695+00:00 host app_name proc_id msg_id ",
        "[exampleSDID@32473 iut=\"3\" eventSource=\"Application\" eventID=\"1011\"]"
    );
    let msg: Result<Message, Error> = Message::parse(msg);

    assert_eq!(
        msg,
        Ok(rsyslog::Message {
            facility: 0,
            severity: 1,
            version: 1,
            timestamp: Some("2021-03-01T19:04:19.887695+00:00"),
            hostname: Some("host"),
            app_name: Some("app_name"),
            proc_id: Some("proc_id"),
            msg_id: Some("msg_id"),
            structured_data: vec![StructuredData {
                id: "exampleSDID@32473",
                params: vec![
                    SdParam {
                        name: "iut",
                        value: "3"
                    },
                    SdParam {
                        name: "eventSource",
                        value: "Application"
                    },
                    SdParam {
                        name: "eventID",
                        value: "1011"
                    }
                ]
            }],
            msg: Raw { msg: "" }
        })
    );
}

#[test]
fn complex_message2() {
    let msg = concat!(
        "<29>1 2016-02-21T04:32:57+00:00 web1 someservice - - ",
        r#"[origin x-service="someservice"][meta sequenceId="14125553"] "#,
        r#"127.0.0.1 - - 1456029177 "GET /v1/ok HTTP/1.1" 200 145 "-" "#,
        r#""hacheck 0.9.0" 24306 127.0.0.1:40124 575"#
    );
    let msg: Result<Message, Error> = Message::parse(msg);

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
                msg: concat!(
                    r#"127.0.0.1 - - 1456029177 "GET /v1/ok HTTP/1.1" "#,
                    r#"200 145 "-" "hacheck 0.9.0" 24306 127.0.0.1:40124 575"#
                )
            }
        })
    );
}

#[test]
fn complex_message3() {
    let msg = concat!(
        "<29>1 2016-02-21T04:32:57+00:00 web1 someservice - - ",
        r#"[origin x-service="someservice"][meta sequenceId="14125553"]"#,
        r#"[origin2 x-service2="Application \"123\""][meta2 sequenceId2="\" 14125553 \""] "#,
        r#"127.0.0.1 - - 1456029177 "GET /v1/ok HTTP/1.1" 200 145 "-" "#,
        r#""hacheck 0.9.0" 24306 127.0.0.1:40124 575"#
    );
    let msg: Result<Message, Error> = Message::parse(msg);

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
                },
                StructuredData {
                    id: "origin2",
                    params: vec![SdParam {
                        name: "x-service2",
                        value: "Application \\\"123\\\""
                    },]
                },
                StructuredData {
                    id: "meta2",
                    params: vec![SdParam {
                        name: "sequenceId2",
                        value: "\\\" 14125553 \\\""
                    },]
                }
            ],
            msg: Raw {
                msg: concat!(
                    r#"127.0.0.1 - - 1456029177 "GET /v1/ok HTTP/1.1" "#,
                    r#"200 145 "-" "hacheck 0.9.0" 24306 127.0.0.1:40124 575"#
                )
            }
        })
    );
}

#[test]
fn heroku_test_message() {
    let msg = concat!(
        "<158>1 2021-03-01T19:04:19.887695+00:00 host heroku router - ",
        "at=info method=POST path=\"/api/v1/events/smartcam\" ",
        "host=ratatoskr.mobility46.se ",
        "request_id=5599e09a-f8e3-4ed9-8be8-6883ce842cf2 ",
        "fwd=\"157.230.107.240\" dyno=web.1 connect=0ms service=97ms ",
        "status=200 bytes=140 protocol=https"
    );
    let msg: Result<Message<_, Skip, HerokuRouter>, Error> = Message::parse(msg);

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
                code: None,
                desc: None,
                method: "POST",
                path: "/api/v1/events/smartcam",
                host: "ratatoskr.mobility46.se",
                request_id: "5599e09a-f8e3-4ed9-8be8-6883ce842cf2",
                fwd: "157.230.107.240",
                dyno: "web.1",
                connect: 0,
                service: 97,
                status: 200,
                bytes: Some(140),
                protocol: "https"
            }
        })
    );
}

#[test]
fn heroku_error_test_message() {
    let msg = concat!(
        "<158>1 2012-10-11T03:47:20+00:00 host heroku router - ",
        r#"at=error code=H12 desc="Request timeout" method=GET path="/" "#,
        "host=myapp.herokuapp.com ",
        "request_id=8601b555-6a83-4c12-8269-97c8e32cdb22 ",
        r#"fwd="204.204.204.204" dyno=web.1 connect=1ms service=30000ms "#,
        "status=503 bytes= protocol=http"
    );
    let msg: Result<Message<_, Skip, HerokuRouter>, Error> = Message::parse(msg);

    assert_eq!(
        msg,
        Ok(rsyslog::Message {
            facility: 19,
            severity: 6,
            version: 1,
            timestamp: Some("2012-10-11T03:47:20+00:00"),
            hostname: Some("host"),
            app_name: Some("heroku"),
            proc_id: Some("router"),
            msg_id: None,
            structured_data: Skip,
            msg: HerokuRouter {
                at: "error",
                code: Some("H12"),
                desc: Some("Request timeout"),
                method: "GET",
                path: "/",
                host: "myapp.herokuapp.com",
                request_id: "8601b555-6a83-4c12-8269-97c8e32cdb22",
                fwd: "204.204.204.204",
                dyno: "web.1",
                connect: 1,
                service: 30000,
                status: 503,
                bytes: None,
                protocol: "http"
            }
        })
    );
}
