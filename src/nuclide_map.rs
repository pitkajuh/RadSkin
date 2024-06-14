use std::collections::HashMap;
use crate::nuclide::NuclideType;
use crate::nuclide::RadioNuclide;
use crate::nuclide::StableNuclide;
use crate::decay::DecayScheme;

use crate::nuclide::NuclideType1;
// use crate::nuclide::RadioNuclideType1;

pub fn get_from_nuclide_map(name: String)
{
    let ba=NuclideType1::RadioNuclideType1(String::from("test"), 30.2);
    println!("aoe {}", ba.half_life);
    let ba_137=NuclideType::StableNuclideType(StableNuclide {
	name: String::from("Ba-137")});
    let ba_137m=NuclideType::RadioNuclideType(RadioNuclide {
	name: String::from("Ba-137m"),
	half_life: 2.6});
    let cs_137=NuclideType::RadioNuclideType(RadioNuclide {
	name: String::from("Cs-137"),
	half_life: 30.2});

    let nuclides: Vec<NuclideType>=vec![ba_137, ba_137m, cs_137];
    let mut nuclide_map: HashMap<String, NuclideType> = HashMap::new();

    // for nuclide in &nuclides {
    // 	nuclide_map.insert(nuclide.name, nuclide);
    // }
}
