use core::f64;

trait Nuclide
{
    // fn exp_law(&self, _time: f64)->f64
    // {
    // 	0.0.to_owned()
    // }

    // fn print_activity(&self, time: f64);

    // fn new(&self);
    // fn new(&self, name: String, half_life: f64, activity: f64)->&dyn Nuclide;
    // fn new(&self);//->&dyn Nuclide;//->Self;

    // fn new()->dyn Nuclide;
    fn new(name: String, half_life: f64, activity: f64)->Self;
}

struct StableNuclide
{
    name: String,
    half_life: f64,
    activity: f64,
}

// struct RadioNuclide
// {
//     name: String,
//     half_life: f64,
//     activity: f64,
// }

impl Nuclide for StableNuclide
{
    fn new(name: String, half_life: f64, activity: f64)->StableNuclide
    {
	StableNuclide{name, half_life, activity}
    }

    // fn new(name: String, half_life: f64, activity: f64) ->StableNuclide
    // {
    // 	StableNuclide{name, half_life, activity}
    // }

    // fn new(&self)
    // fn new(&self)->&dyn Nuclide
    // fn new(&self, name: String, half_life: f64, activity: f64)->StableNuclide
    // fn new(&self, name: String, half_life: f64, activity: f64)->dyn Nuclide
    // // fn new(&self) //->StableNuclide
    // {
    // 	// StableNuclide {name, half_life, activity};
    // 	Nuclide {name, half_life, activity};
    // }

    // fn exp_law(&self, _time: f64)->f64
    // {
    // 	self.activity
    // }

    // fn print_activity(&self, time: f64)
    // {
    // 	println!("Activity of {} is {}", self.name, self.exp_law(time));
    // }

    // fn get_activity(nuclide: &dyn Nuclide, time: f64)
    // {
    // 	nuclide.print_activity(time);
    // }
}

// impl Nuclide for RadioNuclide
// {
//     fn new(&self)
//     {

//     }

//     fn exp_law(&self, time: f64)->f64
//     {
// 	let a: f64=-f64::consts::LN_2*time/self.half_life;
// 	self.activity*a.exp()
//     }

//     fn print_activity(&self, time: f64)
//     {
// 	println!("Activity of {} is {}", self.name, self.exp_law(time));
//     }
// }

    // fn get_activity(nuclide: &dyn Nuclide, time: f64)
    // {
    // 	nuclide.print_activity(time);
    // }


// pub struct Particle
// {
//     energy: f64,
// }

// pub struct Reaction
// {
//     product_nuclide: Nuclide,
//     // product_particle: Particle,
// }

// fn get_activity(radionuclide: &dyn Nuclide, time: f64)->f64
// {
//     return radionuclide.exp_law(time);
// }



// fn print_activity(nuclide: &dyn Nuclide, time: f64)
// {
//     println!("Activity of {} is {} ", nuclide.name, nuclide. get_activity(time));
// }

// fn print_activity(nuclide: RadioNuclide, time: f64)
// {
//     println!("Activity of {} is {} ", nuclide.name, nuclide. get_activity(time));
// }

fn new<T: Nuclide>(name: String, half_life: f64, activity: f64)->T
{
    T::new(name, half_life, activity)
}

// fn new(name: String, half_life: f64, activity: f64)->dyn Nuclide
// {
//     Nuclide(name, half_life, activity);
// }


fn main() {
    println!("Hello, world!");

    // let cs_137=new1(String::from("Cs-137"), 30.0, 50.0);
    // let ba_137m=RadioNuclide {name: String::from("Ba-137m"), half_life: 2.55, activity: 100.0,};
    // let ba_137=StableNuclide {name: String::from("Ba-137"), half_life: 0.0, activity: 100.0,};
    // let ba_137=StableNuclide::new(String::from("Ba-137"), 0.0, 100.0);
    let ba_137=new::<StableNuclide>(String::from("Ba-137"), 0.0, 100.0);

    // get_activity(&cs_137, 10.0);
    // get_activity(&ba_137m, 100.0);
    // get_activity(&ba_137, 10.0);
}

// fn new1(name: String, half_life: f64, activity: f64)->RadioNuclide
// {
//     RadioNuclide
//     {
// 	name,
// 	half_life,
// 	activity,
//     }
// }
