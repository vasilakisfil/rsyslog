use rsyslog::{
    parser::msg::{HerokuRouter, Raw},
    SdParam, StructuredData,
};

#[test]
fn test_simple() {
    let msg = rsyslog::parse::<Raw>("<1>1 - - - - - -");

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
            structured_data: None,
            msg: Raw { msg: "-" }
        })
    );

    let msg = rsyslog::parse::<Raw>("<1>1 2021-03-01T19:04:19.887695+00:00 - - - - -");

    assert_eq!(
        msg,
        Ok(rsyslog::Message {
            facility: 0,
            severity: 1,
            version: 1,
            timestamp: Some(
                chrono::DateTime::parse_from_rfc3339("2021-03-01T19:04:19.887695+00:00").unwrap()
            ),
            hostname: None,
            app_name: None,
            proc_id: None,
            structured_data: None,
            msg: Raw { msg: "-" }
        })
    );

    let msg = rsyslog::parse::<Raw>("<1>1 2021-03-01T19:04:19.887695+00:00 host - - - -");

    assert_eq!(
        msg,
        Ok(rsyslog::Message {
            facility: 0,
            severity: 1,
            version: 1,
            timestamp: Some(
                chrono::DateTime::parse_from_rfc3339("2021-03-01T19:04:19.887695+00:00").unwrap()
            ),
            hostname: Some("host".into()),
            app_name: None,
            proc_id: None,
            structured_data: None,
            msg: Raw { msg: "-" }
        })
    );

    let msg = rsyslog::parse::<Raw>("<1>1 2021-03-01T19:04:19.887695+00:00 host app_name - - -");

    assert_eq!(
        msg,
        Ok(rsyslog::Message {
            facility: 0,
            severity: 1,
            version: 1,
            timestamp: Some(
                chrono::DateTime::parse_from_rfc3339("2021-03-01T19:04:19.887695+00:00").unwrap()
            ),
            hostname: Some("host".into()),
            app_name: Some("app_name".into()),
            proc_id: None,
            structured_data: None,
            msg: Raw { msg: "-" }
        })
    );

    let msg =
        rsyslog::parse::<Raw>("<1>1 2021-03-01T19:04:19.887695+00:00 host app_name proc_id - -");

    assert_eq!(
        msg,
        Ok(rsyslog::Message {
            facility: 0,
            severity: 1,
            version: 1,
            timestamp: Some(
                chrono::DateTime::parse_from_rfc3339("2021-03-01T19:04:19.887695+00:00").unwrap()
            ),
            hostname: Some("host".into()),
            app_name: Some("app_name".into()),
            proc_id: Some("proc_id".into()),
            structured_data: None,
            msg: Raw { msg: "-" }
        })
    );

    let msg = rsyslog::parse::<Raw>(
        "<1>1 2021-03-01T19:04:19.887695+00:00 host app_name proc_id [structured_data] -",
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
            hostname: Some("host".into()),
            app_name: Some("app_name".into()),
            proc_id: Some("proc_id".into()),
            structured_data: Some(vec![StructuredData {
                id: "structured_data",
                params: vec![]
            }]),
            msg: Raw { msg: "-" }
        })
    );
}

#[test]
fn complex_structured_data() {
    let msg = rsyslog::parse::<Raw>("<1>1 2021-03-01T19:04:19.887695+00:00 host app_name proc_id [exampleSDID@32473 iut=\"3\" eventSource=\"Application\" eventID=\"1011\"] -");

    assert_eq!(
        msg,
        Ok(rsyslog::Message {
            facility: 0,
            severity: 1,
            version: 1,
            timestamp: Some(
                chrono::DateTime::parse_from_rfc3339("2021-03-01T19:04:19.887695+00:00").unwrap()
            ),
            hostname: Some("host".into()),
            app_name: Some("app_name".into()),
            proc_id: Some("proc_id".into()),
            structured_data: Some(vec![StructuredData {
                id: "exampleSDID@32473",
                params: vec![
                    SdParam {
                        name: "iut",
                        value: "\"3\""
                    },
                    SdParam {
                        name: "eventSource",
                        value: "\"Application\""
                    },
                    SdParam {
                        name: "eventID",
                        value: "\"1011\""
                    }
                ]
            }]),
            msg: Raw { msg: "-" }
        })
    );
}

#[test]
fn heroku_test_message() {
    let msg = rsyslog::parse::<HerokuRouter>("<158>1 2021-03-01T19:04:19.887695+00:00 host heroku router - at=info method=POST path=\"/api/v1/events/smartcam\" host=ratatoskr.mobility46.se request_id=5599e09a-f8e3-4ed9-8be8-6883ce842cf2 fwd=\"157.230.107.240\" dyno=web.1 connect=0ms service=97ms status=200 bytes=140 protocol=https");

    assert_eq!(
        msg,
        Ok(rsyslog::Message {
            facility: 19,
            severity: 6,
            version: 1,
            timestamp: Some(
                chrono::DateTime::parse_from_rfc3339("2021-03-01T19:04:19.887695+00:00").unwrap()
            ),
            hostname: Some("host"),
            app_name: Some("heroku"),
            proc_id: Some("router"),
            structured_data: None,
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
