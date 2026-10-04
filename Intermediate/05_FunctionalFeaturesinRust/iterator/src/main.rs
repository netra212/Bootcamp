// Generic Type.
// trait Iterator<Item> {
//     // type Item;

//     fn next(&mut self) -> Option<Self::Item>;
// }

// // Implementation block for Generic Type.
// impl Iterator<String> for MyStruct {
//     fn next(&mut self) -> Option<String> {
//         None
//     }
// }

// // Implementation block for Generic Type.
// impl Iterator<i32> for MyStruct {
//     fn next(&mut self) -> Option<i32> {
//         None
//     }
// }

// Associated Type.
// trait Iterator1 {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

// // Iterator.
// //
// trait IntoIterator {
//     type Item;
//     type IntoIter: Iterator1;
//     fn into_iter(self) -> Self::IntoIter; // into_iter() is a method which consume self and return IntoIter or iterator.
// }

// struct MyStruct {}

// impl Iterator1 for MyStruct {
//     type Item = String;
//     fn next(&mut self) -> Option<Self::Item> {
//         None
//     }
// }

// impl Iterator1 for MyStruct {
//     type Item = i32;
//     fn next(&mut self) -> Option<Self::Item> {
//         None
//     }
// }

use std::collections::HashMap;
//
// Implementing Iterator.
//
struct Person {
    first_name: String,
    last_name: String,
    occupation: String,
}

struct PersonIterator {
    values: Vec<String>,
}

impl Iterator for PersonIterator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.values.is_empty() {
            return None;
        }
        Some(self.values.remove(0))
    }
}

impl IntoIterator for Person {
    type Item = String;
    // type IntoIter = PersonIterator;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.first_name, self.last_name, self.occupation].into_iter() // here vector is turning into iterator. 
    }
}

// **************
// Combinators.
// **************
#[derive(Debug)]
struct Student {
    name: String,
    gpa: f32,
}

fn main() {
    // let mut m = MyStruct {};
    // let item: Option<i32> = m.next();
    // let item1: Option<String> = m.next();

    //
    // Implementing `Person` struct.
    //
    let p = Person {
        first_name: "John".to_owned(),
        last_name: "Doe".to_owned(),
        occupation: "Software Engineer".to_owned(),
    };

    // let mut i = p.into_iter(); // into_iter() return an instance of PersonIterator.

    for item in p {
        println!("{item}")
    }

    //
    // Iterator in Collections.
    //
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("red team".to_owned(), 2);
    scores.insert("blue team".to_owned(), 8);
    scores.insert("green team".to_owned(), 6);

    let mut scores_iter = scores.iter(); // iter_mut() method gives an iterator over an mutable reference item in the collection. 
    // iter() method gives an iterator over an immutable reference item in an collection.
    // into_iter() method return an iterator over the owned value in the collection.
    let first = scores_iter.next();

    for (team, score) in &scores {
        println!("{team} Got: {score} points");
    }

    // *****************
    // Combinators.
    // *****************
    let students = vec![
        "Bogdan 3.1",
        "Wallace 2.3",
        "Lidiya 3.5",
        "Kyle 3.9",
        "Anatoliy 4.0",
    ];

    // iter() is an combinator which will return an iterator over the string in the student vector.
    // map() method is a part of combinator. map method convert in an iterator an item from one type to another type.
    let good_students: Vec<Student> = students
        .iter()
        .map(|s| {
            let mut s = s.split(' ');
            let name = s.next()?.to_owned();
            let gpa = s.next()?.parse::<f32>().ok()?;

            Some(Student { name, gpa })
        })
        .flatten()
        .filter(|s| s.gpa >= 3.5)
        .collect();

    // combinator: are small pure function which performs a specific task that could be chain together to perform complex operation.
    // -------------------------------------------------
    let mut good_students = vec![];
    for s in students {
        let mut s = s.split(' ');
        let name = s.next();
        let gpa = s.next();

        if let (Some(name), Some(gpa)) = (name, gpa) {
            let name = name.to_owned();
            let gpa = gpa.parse::<f32>(); // this `::<f32>()` is turbo fish syntax which is used when function defined generics, but unclear what concrete type should subtitute that generic type... () defined with generics type <f23> but don't know what to generics like f. Like what concrete type will be replace or subtitute f. 

            if let Ok(gpa) = gpa {
                if gpa > 3.5 {
                    good_students.push(Student { name, gpa });
                }
            }
        }
    }
}
