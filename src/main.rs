#[derive(Debug)]
#[allow(dead_code)]
enum Color{
    Blue,
    Yellow,
    White,
    Gray,
    Black,
}
struct Dimension{
    height:i32,
    width:i32,
    length:i32,
}
struct ShippingBox{
dimension:Dimension,
color:Color,
weight:i32,
}

impl ShippingBox{
    fn new()->Self{
        Self {
             dimension: ( Dimension { 
                height: 100,
                width: 100,
                length: 100
             }
        ), color: (Color::Black),
         weight: 350 }
    }

    fn print(&self){
        println!("Hi, youre shipping box is of dimension length: {:?}, width: {:?}, height: {:?},\ncolor of {:?} and weight of {:?} kg",
                     self.dimension.length,self.dimension.width,self.dimension.height,self.color,self.weight);
    }
}

fn main(){
    let shipping_box= ShippingBox::new();

    shipping_box.print();
}