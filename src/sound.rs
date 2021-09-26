use derive_more::Display;


#[derive(PartialEq, Eq, Hash, Debug, Clone, Display)]
pub enum SoundType {
    PlayerFire,
    AlienFire,
    PlayerDie,
    AlienDie,
}

impl From<SoundType> for String {
    fn from(sound: SoundType) -> Self {
        sound.to_string()
    }
}
