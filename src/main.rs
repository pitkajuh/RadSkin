mod nuclide;
use crate::nuclide::Nuclide;
use crate::nuclide::StableNuclide;
use crate::nuclide::RadioNuclide;
use crate::nuclide::get_activity;

fn main() {
    println!("Hello, world!");
    let cs_137=RadioNuclide{name: String::from("Cs-137"), half_life: 30.0, activity: 100.0,};
    let ba_137m=RadioNuclide{name: String::from("Ba-137m"), half_life: 2.55, activity: 100.0,};
    let ba_137=StableNuclide{name: String::from("Ba-137"), half_life: 0.0, activity: 100.0};

    get_activity(&cs_137, 30.0);
    get_activity(&ba_137m, 10.0);
    get_activity(&ba_137, 10.0);
}
