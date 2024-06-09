use crate::nuclide::Nuclide;
use crate::nuclide::RadioNuclide;
use crate::nuclide::get_activity;

pub struct Electron
{
    pub energy: f64,
}

// pub struct Decay<'a>
// {
//     pub parent_nuclide: RadioNuclide,
//     pub daughter_nuclide: &'a dyn Nuclide, // Depends on the reaction, whether the produced is stable or unstable.
//     pub product_particle: Vec<Particle>,
// }

pub struct BetaDecay<'a>
{
    pub parent_nuclide: RadioNuclide,
    pub daughter_nuclide: &'a dyn Nuclide, // Depends on the reaction, whether the produced is stable or unstable.
    pub electron: Electron,
    // Antineutrino is omitted.
}
