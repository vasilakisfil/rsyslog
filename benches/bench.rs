use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("rsyslog empty msg", |b| b.iter(|| rsyslog_empty_msg()));
    c.bench_function("syslog_rfc5424 empty msg", |b| {
        b.iter(|| syslog_rfc5424_empty_msg())
    });

    c.bench_function("rsyslog small msg", |b| b.iter(|| rsyslog_small_msg()));
    c.bench_function("syslog_rfc5424 small msg", |b| {
        b.iter(|| syslog_rfc5424_small_msg())
    });

    c.bench_function("rsyslog medium msg", |b| b.iter(|| rsyslog_medium_msg()));
    c.bench_function("syslog_rfc5424 medium msg", |b| {
        b.iter(|| syslog_rfc5424_medium_msg())
    });

    c.bench_function("rsyslog heroku msg", |b| b.iter(|| rsyslog_heroku_msg()));
    c.bench_function("syslog_rfc5424 heroku msg", |b| {
        b.iter(|| syslog_rfc5424_heroku_msg())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

type RsyslogMessage<'a> = rsyslog::Message<
    'a,
    Option<rsyslog::parser::DateTime>,
    Vec<rsyslog::parser::StructuredData<'a>>,
    rsyslog::parser::msg::Raw<'a>,
>;

#[inline]
fn rsyslog_empty_msg<'a>() -> Result<RsyslogMessage<'a>, rsyslog::Error<'a>> {
    rsyslog::Message::parse("<1>1 - - - - - -")
}

#[inline]
fn syslog_rfc5424_empty_msg(
) -> Result<syslog_rfc5424::SyslogMessage, syslog_rfc5424::parser::ParseErr> {
    syslog_rfc5424::parse_message("<1>1 - - - - - -")
}

#[inline]
fn rsyslog_small_msg<'a>() -> Result<RsyslogMessage<'a>, rsyslog::Error<'a>> {
    rsyslog::Message::parse("<1>1 2021-03-01T19:04:19.887695+00:00 host app_name proc_id - -")
}

#[inline]
fn syslog_rfc5424_small_msg(
) -> Result<syslog_rfc5424::SyslogMessage, syslog_rfc5424::parser::ParseErr> {
    syslog_rfc5424::parse_message("<1>1 2021-03-01T19:04:19.887695+00:00 host app_name proc_id - -")
}

#[inline]
fn rsyslog_medium_msg<'a>() -> Result<RsyslogMessage<'a>, rsyslog::Error<'a>> {
    rsyslog::Message::parse(
        r#"<29>1 2016-02-21T04:32:57+00:00 web1 someservice - - [origin x-service="someservice"][meta sequenceId="14125553"] 127.0.0.1 - - 1456029177 "GET /v1/ok HTTP/1.1" 200 145 "-" "hacheck 0.9.0" 24306 127.0.0.1:40124 575"#,
    )
}

#[inline]
fn syslog_rfc5424_medium_msg(
) -> Result<syslog_rfc5424::SyslogMessage, syslog_rfc5424::parser::ParseErr> {
    syslog_rfc5424::parse_message(
        r#"<29>1 2016-02-21T04:32:57+00:00 web1 someservice - - [origin x-service="someservice"][meta sequenceId="14125553"] 127.0.0.1 - - 1456029177 "GET /v1/ok HTTP/1.1" 200 145 "-" "hacheck 0.9.0" 24306 127.0.0.1:40124 575"#,
    )
}

//we add NILVALUE for SD data (the extra "-") since heroku real message skips it
#[inline]
fn rsyslog_heroku_msg<'a>() -> Result<RsyslogMessage<'a>, rsyslog::Error<'a>> {
    rsyslog::Message::parse(
        r#"<158>1 2021-03-01T19:04:19.887695+00:00 host heroku router - - at=info method=POST path=\"/api/v1/events/smartcam\" host=ratatoskr.mobility46.se request_id=5599e09a-f8e3-4ed9-8be8-6883ce842cf2 fwd=\"157.230.107.240\" dyno=web.1 connect=0ms service=97ms status=200 bytes=140 protocol=https"#,
    )
}

//we add NILVALUE for SD data (the extra "-") since heroku real message skips it
#[inline]
fn syslog_rfc5424_heroku_msg(
) -> Result<syslog_rfc5424::SyslogMessage, syslog_rfc5424::parser::ParseErr> {
    syslog_rfc5424::parse_message(
        r#"<158>1 2021-03-01T19:04:19.887695+00:00 host heroku router - - at=info method=POST path=\"/api/v1/events/smartcam\" host=ratatoskr.mobility46.se request_id=5599e09a-f8e3-4ed9-8be8-6883ce842cf2 fwd=\"157.230.107.240\" dyno=web.1 connect=0ms service=97ms status=200 bytes=140 protocol=https"#,
    )
}
