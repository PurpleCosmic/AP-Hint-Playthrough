#[derive(Debug, PartialEq, Clone)]
pub struct Slot {
    pub player: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    pub location: String,
    pub player: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SpoilerEntry {
    pub location: String,
    pub item: String,
    pub sender: String,
    pub receiver: String,
}

impl PartialEq<Location> for SpoilerEntry {
    fn eq(&self, other: &Location) -> bool {
        self.location == other.location && self.sender == other.player
    }
}
impl PartialEq<SpoilerEntry> for Location {
    fn eq(&self, other: &SpoilerEntry) -> bool {
        self.location == other.location && self.player == other.sender
    }
}

impl PartialEq<Location> for Check {
    fn eq(&self, other: &Location) -> bool {
        match self {
            Check::Location(loc) => loc == other,
            Check::Spoiler(spoiler) => spoiler == other,
        }
    }
}

impl PartialEq<SpoilerEntry> for Check {
    fn eq(&self, other: &SpoilerEntry) -> bool {
        match self {
            Check::Location(loc) => loc == other,
            Check::Spoiler(spoiler) => spoiler == other,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Check {
    Spoiler(SpoilerEntry),
    Location(Location),
}

pub type Sphere = Vec<SpoilerEntry>;
pub type Playthrough = Vec<Sphere>;
