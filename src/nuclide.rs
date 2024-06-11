use core::f64;
// use crate::decay::Decay1;
// use crate::decay::DecayType;
// use crate::decay::BetaDecay;

pub trait Nuclide
{
    fn exp_decay(&self, _time: f64)->f64
    {
	0.0.to_owned()
    }

    fn print_activity(&self, time: f64);
    // fn default()->Self;
    // fn new(name: String, half_life: f64, activity: f64)->Self;
}

pub enum NuclideType
{
    RadioNuclideType(RadioNuclide1),
    StableNuclideType(StableNuclide),
}

// pub struct RadioNuclide<'a>
// {
//     pub name: String,
//     pub half_life: f64,
//     pub activity: f64,
//     pub decay: &'a dyn Decay1,
//     pub decay1: DecayType
// }

pub struct RadioNuclide1
{
    pub name: String,
    pub half_life: f64,
    pub activity: f64,
    // pub decay1: Vec<DecayType>
}

pub struct StableNuclide
{
    pub name: String,
}

// impl Nuclide for StableNuclide
// {
//     // fn new(name: String, half_life: f64, activity: f64)->Self
//     // {
//     // 	Self{name, half_life, activity}
//     // }

//     // fn exp_decay(&self, _time: f64)->f64
//     // {
//     // 	self.activity
//     // }

//     // fn print_activity(&self, time: f64)
//     // {
//     // 	println!("Activity of {} is {}", self.name, self.exp_decay(time));
//     // }
// }

impl Nuclide for RadioNuclide1
{
    // fn new(name: String, half_life: f64, activity: f64)->Self
    // {
    // 	Self{name, half_life, activity}
    // }

    fn exp_decay(&self, time: f64)->f64
    {
	let a: f64=-f64::consts::LN_2*time/self.half_life;
	self.activity*a.exp()
    }

    fn print_activity(&self, time: f64)
    {
	println!("Activity of {} is {}", self.name, self.exp_decay(time));
    }
}

// impl <'a> Nuclide for RadioNuclide<'a>
// {
//     // fn default()->Self
//     // {
//     // 	Self{name, half_life, activity, decay: Decay}
//     // }
//     // fn new(name: String, half_life: f64, activity: f64)->Self
//     // {
//     // 	Self{name, half_life, activity}
//     // }

//     fn exp_decay(&self, time: f64)->f64
//     {
// 	let a: f64=-f64::consts::LN_2*time/self.half_life;
// 	self.activity*a.exp()
//     }

//     fn print_activity(&self, time: f64)
//     {
// 	println!("Activity of {} is {}", self.name, self.exp_decay(time));
//     }
// }

// pub fn get_activity(nuclide: &dyn Nuclide, time: f64)
// {
//     nuclide.print_activity(time);
// }
