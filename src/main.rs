use core::f64;

pub struct Particle
{
    energy: f64,
}

pub struct Decay
{
    product_nuclide: RadioNuclide,
    product_particle: Particle,
}

pub struct RadioNuclide
{
    name: String,
    half_life: f64,
    activity: f64,
}

impl RadioNuclide
{
    pub fn exp_law(&self, time: f64)->f64
    {
	let a: f64=-f64::consts::LN_2*time/self.half_life;
	let b: f64=self.activity*a.exp();
	return b;
    }
}

fn main() {
    println!("Hello, world!");

    let cs_137 = new1(String::from("Cs-137"), 30.0, 50.0);
    let ba_137m=RadioNuclide {name: String::from("Ba-137m"), half_life: 2.55/(60.0*365.0), activity: 100.0,};
    let ba_137=RadioNuclide {name: String::from("Ba-137"), half_life: 0.0, activity: 100.0,};

    println!("Activity is {}", cs_137.exp_law(10.0));
    println!("Activity is {}", ba_137m.exp_law(10.0));

}

fn new1(name: String, half_life: f64, activity: f64)->RadioNuclide
{
    RadioNuclide
    {
	name,
	half_life,
	activity,
    }
}
