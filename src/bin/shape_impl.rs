use std::io;
#[derive(Debug)]


struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn perimeter(&self) -> u32{
        2*(self.height + self.width)
    }

    fn can_fit(&self,other:&Self) -> bool{
        self.width > other.width && self.height >other.height
    }
    
    fn create_rectangle(l :u32, b :u32) -> Self {
        Rectangle { width: l, height: h }
    }
    fn create_sq(a:u32) -> Self {
        Rectangle{width: a, height: a }
    }

    fn diag_len(&self) -> u32{
        ((self.width * self.width + self.height * self.height) as f64).sqrt() as u32
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }

    fn get_dimensions(&self) -> (u32,u32){
        (self.width, self.height);
    }



}


fn main(){
    println!("Implementation of Shapes");
    dbg!("Hello console");

}