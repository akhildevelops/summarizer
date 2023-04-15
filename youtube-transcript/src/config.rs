use once_cell::sync::Lazy;
pub struct HTMLParserConfig {
    pub from: &'static str,
    pub to: &'static str,
}
impl Default for HTMLParserConfig {
    fn default() -> Self {
        Self {
            from: "playerCaptionsTracklistRenderer\":",
            to: "},\"videoDetails\"",
        }
    }
}
/// configuration that contains anchor points for identifying captions from youtube's html webpage.
pub struct Config {
    pub(crate) parser: HTMLParserConfig,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            parser: HTMLParserConfig::default(),
        }
    }
}
pub static CONFIG_VAL: Lazy<Config> = Lazy::new(|| Config::default());
