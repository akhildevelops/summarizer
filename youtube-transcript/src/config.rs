use once_cell::sync::Lazy;
use strum_macros::{EnumString, IntoStaticStr};

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

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, Default, EnumString, IntoStaticStr)]
pub enum LangCode {
    /// Arabic
    ar,
    /// Bengali
    bn,
    /// Bulgarian
    bg,
    /// Catalan
    ca,
    /// CN Chinese, Simplified
    zh,
    /// Croatian
    hr,
    /// Czech
    cs,
    /// Danish
    da,
    /// Dutch
    nl,
    /// English
    #[default]
    en,
    /// GB English, UK
    fil,
    /// Finnish
    fi,
    /// French
    fr,
    /// German
    de,
    /// Greek
    el,
    /// Gujarati
    gu,
    /// Hebrew
    iw,
    /// Hindi
    hi,
    /// Hungarian
    hu,
    /// Indonesian
    id,
    /// Italian
    it,
    /// Japanese
    ja,
    /// Kannada
    kn,
    /// Korean
    ko,
    /// Latvian
    lv,
    /// Lithuanian
    lt,
    /// Malay
    ms,
    /// Malayalam
    ml,
    /// Marathi
    mr,
    /// Norwegian
    no,
    /// Polish
    pl,
    /// BR Portuguese, Brazil
    pt,
    /// PT Portuguese, Portugal
    ro,
    /// Russian
    ru,
    /// Serbian
    sr,
    /// Slovak
    sk,
    /// Slovenian
    sl,
    /// Spanish
    es,
    /// Swahili
    sw,
    /// Swedish
    sv,
    /// Tamil
    ta,
    /// Telugu
    te,
    /// Thai
    th,
    /// Turkish
    tr,
    /// Ukrainian
    uk,
    /// Urdu
    ur,
    /// Vietnamese
    vi,
}

/// configuration that contains anchor points for identifying captions from youtube's html webpage.
pub struct Config {
    pub(crate) parser: HTMLParserConfig,
    pub(crate) lang_code: LangCode,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            parser: HTMLParserConfig::default(),
            lang_code: LangCode::default(),
        }
    }
}
