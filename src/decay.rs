use core::f32;
use crate::nuclide::NuclideType;
use crate::nuclide::RadioNuclide;
use crate::nuclide::StableNuclide;

pub enum DecayType<'a> {
    BetaDecayType(BetaDecay<'a>),
    InternalConversionType(InternalConversion<'a>),
}

fn exp_decay(activity: f32, half_life: f32, time: f32) -> f32 {
    let a: f32=-f32::consts::LN_2*time/half_life;
    activity*a.exp()
}

pub struct DecayScheme<'a> {
    pub activity: f32,
    pub decays: Vec<DecayType<'a>>,
    // Return total energy of all decays in MeVs
}

pub struct InternalConversion<'a> {
    pub probability: f32,
    pub parent: &'a RadioNuclide,
    pub daughter: &'a NuclideType
}

pub struct BetaDecay<'a> {
    pub probability: f32,
    pub energy: f32,
    pub parent: &'a RadioNuclide,
    pub daughter: &'a NuclideType
}

trait GetParent {
    fn get_parent(&self) -> &RadioNuclide;
    fn get_daughter(&self) -> &NuclideType;
}

impl <'a>GetParent for BetaDecay<'a> {
    fn get_parent(&self) -> &RadioNuclide {
	&*self.parent
    }

    fn get_daughter(&self) -> &NuclideType {
	&*self.daughter
    }
}

// impl <'a>GetParent for InternalConversion<'a> {
//     fn get_parent(&self) -> &RadioNuclide {
// 	&*self.parent
//     }
// }

fn calc(decay: DecayType, activity: f32, time: f32) {
    // let type_i=decay(_,parent);
    // type_i.parent;
    // let activity_t=exp_decay(activity, decay.)
}
