trait UIComponent {
    fn render(&self) {
        println!("Rendering component...");
    }
}

struct Button {
    text: String,
}

impl UIComponent for Button {}

struct Container {
    name: String,
    child: Box<Container>, // Rust needs to know the size of this container at the compile time. Now, By adding the Box smart pointer, the size of child is exactly equal to the size of Box smart pointer.
}

impl UIComponent for Container {}

fn main() {
    let button_a = Button {
        text: "button a".to_owned(),
    }; // In this way, button_a is stored in stack. 

    let button_b = Box::new {
        // Box Pointer gives single ownerships that store something in heap.
        text: "button b".to_owned(),
    };

    let button_c = button_a; // In this case, whole string will be copied. 
    let button_d = button_b; // Only the box pointer will be copied. 

    // Remember: trait object must be stored between something in this case, we are storing within the Box smart pointer.
    //
    let component: Vec<Box<dyn UIComponent>> = vec![Box::new(button_c), button_d];
}
