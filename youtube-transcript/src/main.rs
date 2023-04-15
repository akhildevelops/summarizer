use clap::{
    builder::{self, IntoResettable},
    Arg, Command,
};
use serde_json;
use youtube_transcript::YoutubeBuilder;

#[derive(Clone)]
enum Format {
    Json,
    Text,
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
            Format::Json => "json".to_string(),
            Format::Text => "text".to_string(),
        }
    }
}
impl TryFrom<&str> for Format {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "json" => Ok(Format::Json),
            "text" => Ok(Format::Text),
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
    let app = Command::new("yts")
        .arg(
            Arg::new("format")
                .help("ouput format")
                .long("format")
                .value_parser(builder::ValueParser::new(format_parser))
                .default_value(Format::Text),
        )
        .arg(Arg::new("link").help("Youtube-link"))
        .get_matches();
    let format = app.get_one::<Format>("format").unwrap_or(&Format::Json);
    let link = app
        .get_one::<String>("link")
        .expect("Youtube Link not provided");
    let transcript = YoutubeBuilder::default()
        .build()
        .transcript(link)
        .await
        .unwrap();

    let data = match format {
        Format::Json => serde_json::to_string(&transcript).unwrap(),
        Format::Text => String::from(transcript),
    };
    println!("{}", data);
}
