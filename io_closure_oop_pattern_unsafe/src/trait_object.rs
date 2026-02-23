
// trait object vs generics
// A generic type parameter can be substituted with only one concrete type at a time
// whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime
// downside of using trait object 1) worse dev exp, need to be explicit about type for vec
// 2) dynamic dispatch meaning worse runtime experience
pub struct Screen {
    pub component: Vec<Box<dyn Draw>>,
}

pub struct Button {
    pub length: usize,
    pub width: usize,
}

impl Draw for Button {
    fn draw(&self) {
        println!("the length of this button is {}, and width is {}", self.length, self.width);
    }
}

pub struct SelectBox {
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("the options for select box are {:?}", self.options);
    }
}

pub struct ScreenV2<T> where T: Draw {
    pub component: Vec<T>,
}

pub trait Draw {
    fn draw(&self);
}

impl Screen {
    pub fn draw(&self) {
        self.component.iter().for_each(|c| c.draw());
    }
}

pub fn build_ui() {
    let s = Screen {
        component:  vec![
            Box::new(Button {length: 1, width: 2}),
            Box::new(SelectBox {options: vec![String::from("op1"), String::from("op2")]}),
        ],
    };
    s.draw();
}
