pub trait Draw{
    fn draw(&self);
}

pub struct Screen{
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen{
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }
}

pub struct Screen2<T: Draw>{
    pub components: Vec<T>,
}

impl<T> Screen2<T>
    where T: Draw{
    pub fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }
}

pub struct Button{
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button{
    fn draw(&self){
        println!("Drawing a button with width: {}, height: {}, and label: {}", self.width, self.height, self.label);
    }
}