use core::f32;

// enum NuclideType2{
//     StableNuclide,
//     RadioNuclide
// }

// struct Nuclide{
//     name: String,
//     nuclide_type: NuclideType2,
//     half_life: f32
// }

// pub enum NuclideType1{
//     RadioNuclideType1(String, f32),
//     StableNuclideType1(String),
// }

// pub struct RadioNuclideType2{
//     pub name: String,
//     pub half_life: f32,
// }

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
