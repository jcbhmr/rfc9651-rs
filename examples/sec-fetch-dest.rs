use derive_more::{Deref, DerefMut, From, Into};
use jcbhmr_rfc9651::Token;
use std::{error::Error, str::FromStr};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ValidSecFetchDest {
    Audio,
    Audioworklet,
    Document,
    Embed,
    Font,
    Frame,
    Iframe,
    Image,
    Json,
    Manifest,
    Object,
    Paintworklet,
    Report,
    Script,
    Serviceworker,
    Sharedworker,
    Style,
    Text,
    Track,
    Video,
    Webidentity,
    Worker,
    Xslt,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, From, Into)]
pub struct SecFetchDest(Option<ValidSecFetchDest>);

impl FromStr for SecFetchDest {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let token: Token = s.parse()?;
        let value = match token.as_str() {
            "audio" => ValidSecFetchDest::Audio,
            "audioworklet" => ValidSecFetchDest::Audioworklet,
            "document" => ValidSecFetchDest::Document,
            "embed" => ValidSecFetchDest::Embed,
            "font" => ValidSecFetchDest::Font,
            "frame" => ValidSecFetchDest::Frame,
            "iframe" => ValidSecFetchDest::Iframe,
            "image" => ValidSecFetchDest::Image,
            "json" => ValidSecFetchDest::Json,
            "manifest" => ValidSecFetchDest::Manifest,
            "object" => ValidSecFetchDest::Object,
            "paintworklet" => ValidSecFetchDest::Paintworklet,
            "report" => ValidSecFetchDest::Report,
            "script" => ValidSecFetchDest::Script,
            "serviceworker" => ValidSecFetchDest::Serviceworker,
            "sharedworker" => ValidSecFetchDest::Sharedworker,
            "style" => ValidSecFetchDest::Style,
            "text" => ValidSecFetchDest::Text,
            "track" => ValidSecFetchDest::Track,
            "video" => ValidSecFetchDest::Video,
            "webidentity" => ValidSecFetchDest::Webidentity,
            "worker" => ValidSecFetchDest::Worker,
            "xslt" => ValidSecFetchDest::Xslt,
            _ => return Ok(SecFetchDest(None)),
        };
        Ok(SecFetchDest(Some(value)))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let header = "document";
    let sec_fetch_dest: SecFetchDest = header.parse()?;
    println!("parsed {:?} into {:?}", &header, &sec_fetch_dest);
    Ok(())
}
