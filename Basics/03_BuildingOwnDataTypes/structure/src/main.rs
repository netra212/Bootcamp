// struct:
struct Product {
    name: String,
    price: f32,
    in_stock: bool,
}

struct NewProduct {
    name: String,
    category: NProductCategory,
    price: f32,
    in_stock: bool,
}

// enums
enum NProductCategory {
    Books,
    Clothing,
    Electrics,
}

// implementation block allows to add the certain functionality to the given type. In this case, for struct Product, if we want to add some functionality then we can use the `impl` block for that.
impl Product {
    // this is also an associated type.
    fn new(name: String, price: f32) -> Product {
        // constructor.
        Product {
            name: name,
            price: price,
            in_stock: true,
        }
    }

    // Associated function types: or static function or static method.
    // associated method associated with other type however they don't work with self.
    // associated function don't takes self as parameter.
    // when calling an associated function, we don't use . syntax.
    fn get_default_sales_tax() -> f32 {
        0.1
    }

    // self -> represent the instance of Product.
    // similar to this: (product: &Product)
    // self here is immutable reference.
    fn calcualte_sales_tax(&self) -> f32 {
        // self.price * 0.1
        // calling get_default_tax() <- associated function type.
        self.price * Product::get_default_sales_tax()
    }

    fn set_price(&mut self, price: f32) {
        self.price = price;
    }

    fn buy(self) -> i32 {
        let name = self.name;
        println!("{name} was bought!");
        123
    }
}

enum Command {
    Undo,
    Redo,
    AddText(String),
    MoveCursor(i32, i32),
    Replace { from: String, to: String },
}

impl Command {
    fn serialize(&self) -> String {
        let json_string = match self {
            Command::Undo => String::from("{ \"cmd\": \"undo\" }"),
            Command::Redo => String::from(
                "
            { \"cmd\": \"redo\" }
        ",
            ),
            Command::AddText(s) => {
                format!(
                    "{{ \
                    \"cmd\": \"add_text\", \
                    \"text\": \"{s}\" \
                }}"
                )
            }
            Command::MoveCursor(line, column) => {
                format!(
                    "{{ \
                        \"cmd\": \"move_cursor\", \
                        \"line\": {line}, \
                        \"column\": {column} \
                    }}"
                )
            }
            Command::Replace { from, to } => {
                format!(
                    "{{ \
                        \"cmd\": \"replace\", \
                        \"from\": \"{from}\", \
                        \"to\": \"{to}\", \
                    }}"
                )
            }
        };
        json_string
    }
}

fn get_username(user_id: u32) -> Option<String> {
    // get username from database.
    let db_result = String::from("Ferris");
    if user_id == 1 { Some(db_result) } else { None }
}

enum Option<T> {
    None,
    Some(T),
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn query_db(query: String) -> Result<String, String> {
    if query.is_empty() {
        Err(String::from("Query string is empty"))
    } else {
        Ok(String::from("Ferris"))
    }
}

fn main() {
    let mut product = Product {
        name: String::from("Book"),
        price: 28.13,
        in_stock: true,
    };

    println!("{}-{}-{}", product.name, product.price, product.in_stock);

    product.calcualte_sales_tax();
    product.set_price(2.0);
    product.buy();

    // tuples.
    let rbg_color = (255, 106, 0);
    let cmyk_color = (0, 58, 100, 0);

    // tuple structs.
    struct RGB(i32, i32, i32);
    struct CMYK(i32, i32, i32, i32);

    let color1 = RGB(255, 106, 0);
    let color2 = CMYK(0, 58, 100, 0);

    // unit-like structs.
    struct MyStruct;

    // ENUMS
    let category = NProductCategory::Electrics;
    let product3 = NewProduct {
        name: String::from("TV"),
        category,
        price: 200.98,
        in_stock: true,
    };

    //
    let cmd = Command::Undo;
    let cmd = Command::AddText(String::from("test"));
    let cmd = Command::MoveCursor(22, 0);
    let cmd = Command::Replace {
        from: String::from("a"),
        to: String::from("b"),
    };

    let json_string = cmd.serialize();

    // match express: allows us to compare series of pattern.
    let age = 35;

    match age {
        1 => println!("Happy 1st Birthday!"),
        13..19 => println!("You just become new voter"),
        x => println!("You are {x} years old...!"),
    }
}

// this function is totally separate from the struct Product.
//
// fn calcualte_sales_tax(product: &Product) -> f32 {
//     product.price * 0.1
// }
