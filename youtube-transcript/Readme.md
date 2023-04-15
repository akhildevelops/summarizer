# Youtube Transcript
Retrieve transcript of any youtube video.

### [Documentation](https://docs.rs/youtube-transcript)

## Install
`cargo install youtube-transcript`

## Usage:


### as Commandline

renders transcript in text format / json format. By default it's text.

``` bash
youtube-transcript https://www.youtube.com/watch?v=RcYjXbSJBN8

start at: 639ms for duration 2s
welcome back
==========


start at: 2s for duration 4s
here we go again great to see you and
==========
...
...
```

For json
``` bash
youtube-transcript --format json https://www.youtube.com/watch?v=RcYjXbSJBN8

{
  "transcripts": [
    {
      "text": "Hey, how&#39;s it going Dave 2d here?",
      "start": {
        "secs": 0,
        "nanos": 0
      },
      "duration": {
        "secs": 1,
        "nanos": 539999962
      }
    },
    {
      "text": "This is a Microsoft Surface go and when they first announced it I was interested in it",
      "start": {
        "secs": 1,
        "nanos": 539999962
      },
      "duration": {
        "secs": 4,
        "nanos": 159999847
      }
    }
    ...
    ...
  ]
}
...
...
```

### as Library
youtube-transcript is an async library and below is the example to use in an applicatio:
``` rust
let link:&str="https://www.youtube.com/watch?v=RcYjXbSJBN8";

# Create a youtube instance from builder.
let youtube_loader:Youtube = YoutubeBuilder::default().build();

# Get the transcript by loading youtube url.
let transcript:Transcript=youtube_loader.transcript(link).await?;
```


### Other tools
Inspired from: [youtube-transcript-api](https://github.com/jdepoix/youtube-transcript-api)
