#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Modification {
    AutoAttack,
    Health,
    Radiance,
    Splash,
    Stars,
    Lightning
}

pub struct ChooseModificationEvent {
    pub modification: Modification,
}
