use std::hash::{Hash, Hasher};

/// The decoded contents of a frame.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Content {
    /// A value containing the parsed contents of a text frame.
    Text(String),
    /// A value containing the parsed contents of a user defined text frame (TXXX).
    ExtendedText(ExtendedText),
    /// A value containing the parsed contents of a web link frame.
    Link(String),
    /// A value containing the parsed contents of a user defined web link frame (WXXX).
    ExtendedLink(ExtendedLink),
    /// A value containing the parsed contents of a comment frame (COMM).
    Comment(Comment),
    /// A value containing the parsed contents of a lyrics frame (USLT).
    Lyrics(Lyrics),
    /// A value containing the parsed contents of a synchronised lyrics frame (SYLT).
    SynchronisedLyrics(SynchronisedLyrics),
    /// A value containing the parsed contents of a picture frame (APIC).
    Picture(Picture),
    /// A value containing the bytes of a unknown frame.
    Unknown(Vec<u8>),
}

impl Content {
    /// Returns the `Text` or None if the value is not `Text`.
    pub fn text(&self) -> Option<&str> {
        match *self {
            Content::Text(ref content) => Some(&*content),
            _ => None,
        }
    }

    /// Returns the `ExtendedText` or None if the value is not `ExtendedText`.
    pub fn extended_text(&self) -> Option<&ExtendedText> {
        match *self {
            Content::ExtendedText(ref content) => Some(content),
            _ => None,
        }
    }

    /// Returns the `Link` or None if the value is not `Link`.
    pub fn link(&self) -> Option<&str> {
        match *self {
            Content::Link(ref content) => Some(content),
            _ => None,
        }
    }

    /// Returns the `ExtendedLink` or None if the value is not `ExtendedLink`.
    pub fn extended_link(&self) -> Option<&ExtendedLink> {
        match *self {
            Content::ExtendedLink(ref content) => Some(content),
            _ => None,
        }
    }

    /// Returns the `Comment` or None if the value is not `Comment`.
    pub fn comment(&self) -> Option<&Comment> {
        match *self {
            Content::Comment(ref content) => Some(content),
            _ => None,
        }
    }

    /// Returns the `Lyrics` or None if the value is not `Lyrics`.
    pub fn lyrics(&self) -> Option<&Lyrics> {
        match *self {
            Content::Lyrics(ref content) => Some(content),
            _ => None,
        }
    }

    /// Returns the `SynchronisedLyrics` or None if the value is not `SynchronisedLyrics`.
    pub fn synchronised_lyrics(&self) -> Option<&SynchronisedLyrics> {
        match *self {
            Content::SynchronisedLyrics(ref content) => Some(content),
            _ => None,
        }
    }

    /// Returns the `Picture` or None if the value is not `Picture`.
    pub fn picture(&self) -> Option<&Picture> {
        match *self {
            Content::Picture(ref picture) => Some(picture),
            _ => None,
        }
    }

    /// Returns the `Unknown` or None if the value is not `Unknown`.
    pub fn unknown(&self) -> Option<&[u8]> {
        match *self {
            Content::Unknown(ref data) => Some(&data[..]),
            _ => None,
        }
    }
}

/// The parsed contents of an extended text frame.
#[derive(Clone, Debug, Eq)]
#[allow(missing_docs)]
pub struct ExtendedText {
    pub description: String,
    pub value: String,
}

impl PartialEq for ExtendedText {
    fn eq(&self, other: &Self) -> bool {
        self.description == other.description
    }
}

impl Hash for ExtendedText {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.description.hash(state);
    }
}

/// The parsed contents of an extended link frame.
#[derive(Clone, Debug, Eq)]
#[allow(missing_docs)]
pub struct ExtendedLink {
    pub description: String,
    pub link: String,
}

impl PartialEq for ExtendedLink {
    fn eq(&self, other: &Self) -> bool {
        self.description == other.description
    }
}

impl Hash for ExtendedLink {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.description.hash(state);
    }
}

/// The parsed contents of a comment frame.
#[derive(Clone, Debug, Eq)]
#[allow(missing_docs)]
pub struct Comment {
    pub lang: String,
    pub description: String,
    pub text: String,
}

impl PartialEq for Comment {
    fn eq(&self, other: &Self) -> bool {
        self.lang == other.lang && self.description == other.description
    }
}

impl Hash for Comment {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lang.hash(state);
        self.description.hash(state);
    }
}

/// The parsed contents of an unsynchronized lyrics frame.
#[derive(Clone, Debug, Eq)]
#[allow(missing_docs)]
pub struct Lyrics {
    pub lang: String,
    pub description: String,
    pub text: String,
}

impl PartialEq for Lyrics {
    fn eq(&self, other: &Self) -> bool {
        self.lang == other.lang && self.description == other.description
    }
}

impl Hash for Lyrics {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lang.hash(state);
        self.description.hash(state);
    }
}

/// The parsed contents of an unsynchronized lyrics frame.
#[derive(Clone, Debug, Eq)]
#[allow(missing_docs)]
pub struct SynchronisedLyrics {
    pub lang: String,
    pub timestamp_format: TimestampFormat,
    pub content_type: SynchronisedLyricsType,
    // The content of a synchronised lyrics consists of the text segments mapped to a timestamp as
    // specified by the `timestamp_format` field.
    pub content: Vec<(u32, String)>,
}

impl PartialEq for SynchronisedLyrics {
    fn eq(&self, other: &Self) -> bool {
        self.lang == other.lang && self.content_type == other.content_type
    }
}

impl Hash for SynchronisedLyrics {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lang.hash(state);
        self.content_type.hash(state);
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum TimestampFormat {
    // Absolute time, using MPEG frames as unit.
    MPEG,
    // Absolute time, using milliseconds as unit.
    MS,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum SynchronisedLyricsType {
    // Is other.
    Other,
    // Is lyrics.
    Lyrics,
    // Is text transcription.
    Transcription,
    // Is movement/part name (e.g. "Adagio").
    PartName,
    // Is events (e.g. "Don Quijote enters the stage").
    Event,
    // Is chord (e.g. "Bb F Fsus").
    Chord,
    // Is trivia/'pop up' information.
    Trivia,
}

/// Types of pictures used in APIC frames.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum PictureType {
    Other,
    Icon,
    OtherIcon,
    CoverFront,
    CoverBack,
    Leaflet,
    Media,
    LeadArtist,
    Artist,
    Conductor,
    Band,
    Composer,
    Lyricist,
    RecordingLocation,
    DuringRecording,
    DuringPerformance,
    ScreenCapture,
    BrightFish,
    Illustration,
    BandLogo,
    PublisherLogo,
    Undefined(u8),
}

impl From<PictureType> for u8 {
    fn from(pt: PictureType) -> Self {
        match pt {
            PictureType::Other => 0,
            PictureType::Icon => 1,
            PictureType::OtherIcon => 2,
            PictureType::CoverFront => 3,
            PictureType::CoverBack => 4,
            PictureType::Leaflet => 5,
            PictureType::Media => 6,
            PictureType::LeadArtist => 7,
            PictureType::Artist => 8,
            PictureType::Conductor => 9,
            PictureType::Band => 10,
            PictureType::Composer => 11,
            PictureType::Lyricist => 12,
            PictureType::RecordingLocation => 13,
            PictureType::DuringRecording => 14,
            PictureType::DuringPerformance => 15,
            PictureType::ScreenCapture => 16,
            PictureType::BrightFish => 17,
            PictureType::Illustration => 18,
            PictureType::BandLogo => 19,
            PictureType::PublisherLogo => 20,
            PictureType::Undefined(b) => b,
        }
    }
}

/// A structure representing an ID3 picture frame's contents.
#[derive(Clone, Eq, Debug)]
pub struct Picture {
    /// The picture's MIME type.
    pub mime_type: String,
    /// The type of picture.
    pub picture_type: PictureType,
    /// A description of the picture's contents.
    pub description: String,
    /// The image data.
    pub data: Vec<u8>,
}

impl PartialEq for Picture {
    fn eq(&self, other: &Self) -> bool {
        self.picture_type == other.picture_type
    }
}

impl Hash for Picture {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.picture_type.hash(state);
    }
}
