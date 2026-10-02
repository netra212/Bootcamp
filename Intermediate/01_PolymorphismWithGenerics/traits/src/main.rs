trait Park {
    // In trait, only functionality can be shared not the actual data. Since, trait is like a interface.
    fn park(&self);
}

trait Paint {
    fn paint(&self, color: String) {
        println!("painting objec: {}", color);
    }
}

struct VechicleInfo {
    make: String,
    model: String,
    year: u16,
}

struct Car {
    info: VechicleInfo,
}

impl Park for Car {
    fn park(&self) {
        println!("parking car.!");
    }
}

impl Paint for Car {} // Now, we are implementing Paint trait for Car. 
struct Truck {
    info: VechicleInfo,
}

impl Truck {
    fn unload(&self) {
        println!("Unloading truck.")
    }
}

impl Park for Truck {
    fn park(&self) {
        println!("parking truck.!");
    }
}

impl Paint for Truck {}

struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        // Now, here we are overriding the function. or function overriding.
        println!("painting house: {}", color);
    }
}

fn main() {
    //
    let car = Car {
        info: VechicleInfo {
            make: "Honda".to_owned(),
            model: "Civic".to_owned(),
            year: 1995,
        },
    };

    let house = House {};
    let object = create_paintable_object();
    paint_red(&car);
    paint_red(&house);
    paint_red(&object);

    paint_vechicle_red(&car);
}

// Below is the showcase of implementing the trait_bounds in rust.. because we are bounding `Paint` trait with generice type T.
fn paint_red<T: Paint>(object: &T) {
    object.paint("red".to_owned());
}

// This below code is simply syntatic sugar of the above paint bound.
// here, &impl means object must be a reference that something implement Paint trait.
fn paint_red2(object: &impl Paint) {
    object.paint("red".to_owned());
}

// with where clause
fn paint_vechicle_red<T>(object: &T)
where
    T: Paint + Park, // Here T is implementing both Paint and Park trait.
{
    object.paint("red".to_owned());
}

fn create_paintable_object() -> impl Paint {
    // return type must be something that implement Paint trait type.
    House {}
}
