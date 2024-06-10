use crate::nuclide::Nuclide;
use crate::nuclide::RadioNuclide;
// use crate::nuclide::get_activity;

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

pub trait Decay1
{


}


enum DecayType
{
    BetaDecay,
}

pub struct Decay
{
    decay: DecayType,
}

pub struct BetaDecay<'a>
{
    pub parent_nuclide: RadioNuclide<'a>,
    pub daughter_nuclide: Vec<&'a dyn Nuclide>,
    pub electron: Vec<Electron>
}

// pub struct BetaDecay<'a>
// {
//     pub parent_nuclide: RadioNuclide<'a>,
//     pub daughter_nuclide: Vec<&'a dyn Nuclide>,
//     pub electron: Vec<Electron>
// }

// pub struct DecayScheme
// {


// }

// pub fn beta_decay(parent: RadioNuclide)
// {

// }

// pub fn new_decay(parent: &dyn Nuclide)
// {
// // If parent is RadioNuclide, calculate decay, otherwise skip.

// }
