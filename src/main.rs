mod nuclide;
mod decay;
// // use crate::nuclide::Nuclide;
use crate::nuclide::StableNuclide;
// use crate::nuclide::RadioNuclide;
// // use crate::nuclide::get_activity;
// use crate::decay::BetaDecay;
use crate::decay::BetaDecayTest;
use crate::decay::InternalConversion;
use crate::nuclide::NuclideType;
use crate::nuclide::RadioNuclide1;

fn main() {
    println!("Hello, world!");
    const half_life: f64=30.2;
    let activity: f64=100.0;
    let cs_137=RadioNuclide1{name: String::from("Cs-137"), half_life: half_life, activity: activity};
    let ba_137m=RadioNuclide1{name: String::from("Ba-137m"), half_life: 2.6, activity: 100.0};
    let ba_137=StableNuclide{name: String::from("Ba-137")};

    let parent1=NuclideType::RadioNuclideType(cs_137);
    let daughter1=NuclideType::StableNuclideType(ba_137);
    let decay1=BetaDecayTest{probability: 0.054, parent: parent1, daughter: &daughter1};
    let decay2=InternalConversion{probability: 0.946, parent: parent1, daughter: &daughter1};
    // Subtract from activity variable, then calculate next decay branch.
    let parent2=NuclideType::RadioNuclideType(ba_137m);
    let decay3=InternalConversion{probability: 1.0, parent: parent2, daughter: &daughter1};


    // let cs_137=RadioNuclide1{name: String::from("Cs-137"), half_life: 30.2, activity: 100.0};

    // let beta_decay: BetaDecay;
    // let cs_137=RadioNuclide{name: String::from("Cs-137"), half_life: 30.2, activity: 100.0, decay: &beta_decay};
    // let cs_137=RadioNuclide{name: String::from("Cs-137"), half_life: 30.2, activity: 100.0};

    // let ba_137m=RadioNuclide{name: String::from("Ba-137m"), half_life: 2.6, activity: 100.0};
    // let ba_137=StableNuclide{name: String::from("Ba-137"), half_life: 0.0, activity: 100.0};

    // let co_60=RadioNuclide{name: String::from("Co-60"), half_life: 5.3, activity: 100.0};
    // let ni_60=StableNuclide{name: String::from("Ni-60"), half_life: 0.0, activity: 100.0};

    // get_activity(&cs_137, 30.0);
    // get_activity(&ba_137m, 10.0);
    // get_activity(&ba_137, 10.0);

    // get_activity(&co_60, 30.0);
    // get_activity(&ni_60, 10.0);
}
