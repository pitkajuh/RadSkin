use crate::nuclide::Nuclide;
use crate::nuclide::RadioNuclide;
use crate::nuclide::get_activity;

// pub trait Particle
// {

// }

pub struct Electron
{
    pub energy: f64,
}

pub struct Gamma
{
    pub energy: f64,
}

pub trait Decay
{


}

pub struct BetaDecay<'a>
{
    pub parent_nuclide: RadioNuclide,
    pub daughter_nuclide: Vec<&'a dyn Nuclide>,
    pub electron: Vec<Electron>
}

pub struct DecayScheme
{


}
