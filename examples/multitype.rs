use rsyslog::{
    parser::{
        msg::{HerokuRouter, LineRaw},
        Skip,
    },
    Error, Message,
};

enum CustomRaw<'a> {
    Router(HerokuRouter<'a>),
    Other(LineRaw<'a>),
}

impl<'a> From<HerokuRouter<'a>> for CustomRaw<'a> {
    fn from(parser: HerokuRouter<'a>) -> Self {
        Self::Router(parser)
    }
}

impl<'a> From<LineRaw<'a>> for CustomRaw<'a> {
    fn from(parser: LineRaw<'a>) -> Self {
        Self::Other(parser)
    }
}

impl<'a> rsyslog::ParseMsg<'a> for CustomRaw<'a> {
    fn parse(
        msg: &'a str,
        originator: &rsyslog::Originator<'a>,
    ) -> Result<(&'a str, Self), Error<'a>> {
        match originator.proc_id {
            Some("router") => {
                let (rem, message) = HerokuRouter::parse(msg, originator)?;
                //println!("{}", rem);
                //let (rem, _) = LineRaw::parse(rem, &originator)?;
                Ok((rem, message.into()))
            }
            _ => LineRaw::parse(msg, originator).map(|(s, msg)| (s, msg.into())),
        }
    }
}

type MultiType<'a> = Message<'a, Option<&'a str>, Skip, CustomRaw<'a>>;

fn main() -> Result<(), String> {
    let msg = r#"284 <158>1 2021-03-01T19:04:19.887695+00:00 host heroku router - at=info method=POST path="/api/v1/events/smartcam" host=ratatoskr.mobility46.se request_id=5599e09a-f8e3-4ed9-8be8-6883ce842cf2 fwd="157.230.107.240" dyno=web.1 connect=0ms service=97ms status=200 bytes=140 protocol=https
231 <190>1 2021-02-25T13:04:29.326809+00:00 host app web.1 - 2021-02-25T13:04:29.326 INFO  api::smartcam_events                > Low confidence event `n.a.` with confidence 0 (minimum 10) from camera FXMCU_CM_FE_02199902A/1:002137214"#;

    /*
    println!(
        "{:?}",
        MultiType::iter(msg).map(|s| s.proc_id).collect::<Vec<_>>()
    );
    */

    for line in MultiType::iter(msg) {
        let line = line.map_err(|s| s.to_string())?;

        println!("{:?}", line.proc_id);
        match line.msg {
            CustomRaw::Router(heroku_router) => println!("{heroku_router:?}"),
            CustomRaw::Other(line_raw) => println!("{line_raw:?}"),
        }
    }

    Ok(())
}
