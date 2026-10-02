// In Rust, one trait can rely on other trait which is called super traits.

// super trait are super useful that rely on functionality of another trait.

trait Vechicle: Paint {
    // This mean, any type implementing the Vechicle trait also must implement the Paint trait.
    fn park(&self);
    // trait can also contain associated function.
    fn get_default_color() -> String {
        "black".to_owned()
    }
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
    let object = create_paintable_object(true);
    paint_red(&car);
    paint_red(&house);
    // paint_red(&object);
    paint_red(object.as_ref());

    paint_vechicle_red(&car);

    let paintable_object: Vec<dyn Paint> = vec![&car, &house];
}

// Below is the showcase of implementing the trait_bounds in rust.. because we are bounding `Paint` trait with generice type T.
// fn paint_red<T: Paint>(object: &T) {
//     object.paint("red".to_owned());
// }

fn paint_red(object: &dyn Paint) {
    object.paint("Red".to_owned())
}

// This below code is simply syntatic sugar of the above paint bound.
// here, &impl means object must be a reference that something implement Paint trait.
fn paint_red2(object: &impl Paint) {
    object.paint("red".to_owned());
}

// with where clause
fn paint_vechicle_red<T>(object: &T)
where
    T: Vechicle, // Here T is implementing both Paint and Park trait.
{
    object.paint("red".to_owned());
}

// returing trait bound.
// since, we are returning with two concrete type but in compile type two concrete type must be converted into one type so that we have implemented Box<dyn Pain> which return the trait object.
// trait object allows us to defied a type without knowing what that type is compile type.
// trait object is defined with `dyn` keyword which means dynamic dispatched and must be behind with some type of pointer. In this case, Box pointer is implemented.
fn create_paintable_object(vechicle: bool) -> Box<dyn Paint> {
    // return type must be something that implement Paint trait type.

    if vechicle {
        // If block is returning Car.
        Box::new(Car {
            info: VechicleInfo {
                make: "Honda".to_owned(),
                model: "Civic".to_owned(),
                year: 1995,
            },
        })
    } else {
        // else block is returning House.
        Box::new(House {})
    }
}

// Difference between static Dispatch Vs Dynamic Dispatch.
//
// static Dispatch:
// when the compiler knows which concrete method to call at the compile type. For example:

// Dynamic Dispatch:
// When the compiler does not know which concrete method to call at the compile time but it insert lit bit of code to figure that out at run time... Like this... Box<dyn Paint>
// The advantages of dynamic dispatch is flexibility, we can return anything or any object that implement `Paint`
// Disadvantage is runtime performance cost.

// trait object is used when compiler does not know which concrete type will be used at the compile type. eg: Box<dyn Paint>
// Another situation of trait object is being used is when creating a collection of type that implement certain trait.
