# Rsyslog
Very flexible Rust library for parsing syslog based on [RFC 5424](https://tools.ietf.org/html/rfc5424).
Uses [nom](https://github.com/Geal/nom) as the sole dependency.

## Features
* This thing is _fast_. It is 50% faster than [rust-syslog-rfc5424](https://github.com/Roguelazer/rust-syslog-rfc5424)
if you are ok with an `Option<&str>` for TIMESTAMP.
And they are on pair when having the `chrono-timestamp` feature on (parses TIMESTAMP
as chrono `DateTime<Offset>` type).
Compared to any Ruby/Python/Js implementation is obviously an order of magnitude faster.
It's not super optimized for performance (especially around SD) and I suspect that
[rust-syslog-rfc5424](https://github.com/Roguelazer/rust-syslog-rfc5424) is not either.
In any case, performance isn't the main goal of rsyslog. It's flexibility.
* it allows you to inject your own TIMESTAMP, SD and MSG parsers in case of a need.
All you need is to implement the necessary traits and inject the type when you
specify the parser type. In my case I needed to parse a syslog that has invalid
SD (basically it's not there at all, looking at you Heroku) and a msg parser that
is capable of parsing raw messages as well as Heroku router message format.
You can see an example in [example/multitype](example/multitype.rs).
* It allows you to parse on the fly multiple messages, without having to traverse
the initial string all the way to find the breakpoints (usually new line) and then
breakit into substrings in a Vec.
All you need is to inject a MSG parser that will make sure it stops when it
should stop. Then you can use the iterator that will allow you to iterate and
parse each substring.
* Rsyslog provides some common implementations of some TIMESTAMP, STRUCTURED DATA
and MSG parsers.

#### Cargo features
Optional features:
* `chrono-timestamp`: Allows you to parse TIMESTAMP as `Option<chrono::DateTime<chrono::FixedOffset>>`.
* `serde-serialize`: Allows you to serialize the Message struct using serde.

## Example of usage

### Simple message
```rust
let msg = r#"<29>1 2016-02-21T04:32:57+00:00 web1 someservice - - [origin x-service="someservice"][meta sequenceId="14125553"] 127.0.0.1 - - 1456029177 "GET /v1/ok HTTP/1.1" 200 145 "-" "hacheck 0.9.0" 24306 127.0.0.1:40124 575"#;
let message: Message = rsyslog::Message::parse(msg)?;
```

By default Message type is `Message<'a, Option<&'a str>, Vec<StructuredData>, Raw<'a>>`
using default generic type params.

### Multiline message
```rust
type OneLineMessage<'a> = Message<'a, Option<&'a str>, Vec<StructuredData<'a>>, LineRaw<'a>>;

let msg = r#"<29>1 2016-02-21T04:32:57+00:00 web1 someservice - - - 127.0.0.1 - - 1456029177 "GET /v1/info HTTP/1.1" 200 145 "-" "hacheck 0.9.0" 24306 127.0.0.1:40124 575
<29>1 2016-02-21T05:32:57+00:00 web2 someservice - - - 127.0.0.1 - - 1456029177 "GET /v1/videos HTTP/1.1" 200 145 "-" "hacheck 0.9.0" 24306 127.0.0.1:40124 575
<29>1 2016-02-21T06:32:57+00:00 web3 someservice - - - 127.0.0.1 - - 1456029177 "GET /v1/users HTTP/1.1" 200 145 "-" "hacheck 0.9.0" 24306 127.0.0.1:40124 575"#;

let hostnames = OneLineMessage::iter(msg)
    .map(|s| s.map(|s| s.hostname))
    .collect::<Vec<_>>();
```

You can find more examples in the [examples](examples/) directory.
