mod nuclide;
mod decay;
use crate::nuclide::NuclideType;
use crate::nuclide::RadioNuclide;
use crate::nuclide::StableNuclide;
use crate::decay::DecayScheme;
use crate::decay::DecayType;
use crate::decay::BetaDecay;
use crate::decay::InternalConversion;

fn main() {
    const HALF_LIFE: f32=30.2;
    let activity: f32=100.0;

    let cs_137=RadioNuclide{
	name: String::from("Cs-137"),
	half_life: HALF_LIFE};
    let ba_137m=RadioNuclide{
	name: String::from("Ba-137m"),
	half_life: 2.6};
    let ba_137=StableNuclide{
	name: String::from("Ba-137")};

    // let parent1=NuclideType::RadioNuclideType(cs_137);
    let daughter1=NuclideType::StableNuclideType(ba_137);
    // let parent2=NuclideType::RadioNuclideType(ba_137m);

    let decay1=BetaDecay{
	probability: 0.054,
	parent: &cs_137,
	daughter: &daughter1};
    // let decay2=BetaDecay{probability: 0.946, parent: &cs_137, daughter: &daughter1};
    // let decay3=InternalConversion{probability: 1.0, parent: &ba_137m, daughter: &daughter1};

    let cs_137_branch_1=DecayType::BetaDecayType(decay1);
    // let cs_137_branch_2=DecayType::BetaDecayType(decay2);
    // let cs_137_branch_3=DecayType::InternalConversionType(decay3);

    // let decays=vec![cs_137_branch_1, cs_137_branch_2, cs_137_branch_3];
    // let cs_137_decay=DecayScheme{activity: activity, decays: decays};

    // for i in &decays
    // {


    // }

}
