use std::error::Error;

use clap::{
    builder::{self, IntoResettable},
    Arg, Command,
};
use reqwest::multipart::Form;
use serde_json::value;
use youtube_transcript::{Config, Youtube};

#[derive(Clone)]
enum Format {
    json,
    text,
}
impl IntoResettable<builder::OsStr> for Format {
    fn into_resettable(self) -> builder::Resettable<builder::OsStr> {
        let format_str = self.to_string();
        <String as Into<builder::OsStr>>::into(format_str).into()
    }
}

impl ToString for Format {
    fn to_string(&self) -> String {
        match self {
            Format::json => "json".to_string(),
            Format::text => "text".to_string(),
        }
    }
}
impl TryFrom<&str> for Format {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "json" => Ok(Format::json),
            "text" => Ok(Format::text),
            _ => Err(format!(
                "Cannot find json / text as format definition. Recieved {}",
                value
            )),
        }
    }
}

fn format_parser(arg: &str) -> Result<Format, String> {
    arg.try_into()
}

#[tokio::main]
async fn main() {
    let config = Config::default();
    let app = Command::new("yts")
        .arg(
            Arg::new("format")
                .help("ouput format")
                .long("format")
                .value_parser(builder::ValueParser::new(format_parser))
                .default_value(Format::text),
        )
        .arg(Arg::new("link").help("Youtube-link"))
        .get_matches();
    let format = app.get_one::<Format>("format").unwrap_or(&Format::json);
    let link = app
        .get_one::<String>("link")
        .expect("Youtube Link not provided");
    let transcript = Youtube::link(link, &config).get_transcript().await.unwrap();

    println!("{}", String::from(transcript));
}
