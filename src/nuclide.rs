use core::f64;

pub trait Nuclide
{
    fn exp_law(&self, _time: f64)->f64
    {
	0.0.to_owned()
    }
    fn print_activity(&self, time: f64);
    // fn new(name: String, half_life: f64, activity: f64)->Self;
}

pub struct RadioNuclide
{
    pub name: String,
    pub half_life: f64,
    pub activity: f64,
}

pub struct StableNuclide
{
    pub name: String,
    pub half_life: f64,
    pub activity: f64,
}

impl Nuclide for StableNuclide
{
    // fn new(name: String, half_life: f64, activity: f64)->StableNuclide
    // {
    // 	StableNuclide2{name, half_life, activity}
    // }

    fn exp_law(&self, _time: f64)->f64
    {
	self.activity
    }

    fn print_activity(&self, time: f64)
    {
	println!("Activity of {} is {}", self.name, self.exp_law(time));
    }
}

impl Nuclide for RadioNuclide
{
    fn exp_law(&self, time: f64)->f64
    {
	let a: f64=-f64::consts::LN_2*time/self.half_life;
	self.activity*a.exp()
    }

    fn print_activity(&self, time: f64)
    {
	println!("Activity of {} is {}", self.name, self.exp_law(time));
    }
}

pub fn get_activity(nuclide: &dyn Nuclide, time: f64)
{
    nuclide.print_activity(time);
}
