use core::f32;
use crate::nuclide::NuclideType;

pub enum DecayType<'a>
{
    BetaDecayType(BetaDecay<'a>),
    InternalConversionType(InternalConversion<'a>),
}

fn exp_decay(activity: f32, half_life: f32, time: f32)->f32
{
    let a: f32=-f32::consts::LN_2*time/half_life;
    activity*a.exp()
}

pub struct DecayScheme<'a>
{
    pub activity: f32,
    pub decays: Vec<DecayType<'a>>,
    // Return total energy of all decays in MeVs
}

pub struct InternalConversion<'a>
{
    pub probability: f32,
    pub parent: &'a NuclideType,
    pub daughter: &'a NuclideType,
}

pub struct BetaDecay<'a>
{
    pub probability: f32,
    pub parent: &'a NuclideType,
    pub daughter: &'a NuclideType,
}
