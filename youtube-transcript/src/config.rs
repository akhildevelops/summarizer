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
pub struct Config {
    pub parser: HTMLParserConfig,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            parser: HTMLParserConfig::default(),
        }
    }
}
pub static CONFIG_VAL: Lazy<Config> = Lazy::new(|| Config::default());
