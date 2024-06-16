use core::f32;

pub enum NuclideType {
    RadioNuclideType(RadioNuclide),
    StableNuclideType(StableNuclide),
}

pub struct RadioNuclide {
    pub name: String,
    pub half_life: f32,
}

pub struct StableNuclide {
    pub name: String,
}
