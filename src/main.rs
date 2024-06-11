mod nuclide;
mod decay;
use std::vec::Vec;
// // use crate::nuclide::Nuclide;
use crate::nuclide::StableNuclide;
// use crate::nuclide::RadioNuclide;
// // use crate::nuclide::get_activity;
// use crate::decay::BetaDecay;
use crate::decay::DecayType;
use crate::decay::DecayScheme;
use crate::decay::BetaDecayTest;
use crate::decay::InternalConversion;
use crate::nuclide::NuclideType;
use crate::nuclide::RadioNuclide1;

fn main() {
    println!("Hello, world!");
    const HALF_LIFE: f64=30.2;
    let activity: f64=100.0;
    let cs_137=RadioNuclide1{name: String::from("Cs-137"), half_life: HALF_LIFE, activity: activity};
    let ba_137m=RadioNuclide1{name: String::from("Ba-137m"), half_life: 2.6, activity: 100.0};
    let ba_137=StableNuclide{name: String::from("Ba-137")};

    let parent1=NuclideType::RadioNuclideType(cs_137);
    let daughter1=NuclideType::StableNuclideType(ba_137);
    let parent2=NuclideType::RadioNuclideType(ba_137m);

    let decay1=BetaDecayTest{probability: 0.054, parent: &parent1, daughter: &daughter1};
    let decay2=BetaDecayTest{probability: 0.946, parent: &parent1, daughter: &daughter1};
    // Subtract from activity variable, then calculate next decay branch.
    let decay3=InternalConversion{probability: 1.0, parent: &parent2, daughter: &daughter1};

    let cs_137_branch_1=DecayType::BetaDecayTestType(decay1);
    let cs_137_branch_2=DecayType::BetaDecayTestType(decay2);
    let cs_137_branch_3=DecayType::InternalConversionType(decay3);

    let decays: Vec<DecayType>=vec![cs_137_branch_1, cs_137_branch_2, cs_137_branch_3];
    let cs_137_decay=DecayScheme{decays: decays};
}
