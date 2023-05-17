fn main{
    let c=Calculator:: Add(3,5);
    calculate(c);
}

enum Calculator{
    Add{x: f64, y: f64},
    Sub{x: f64, y: f64},
    Mul{x: f64, y: f64},
    Div{x: f64, y: f64},
}

fn calculate(cal: Calculator){
    match cal{
        Calculator::Add {x,y} => {
            let res = x + y;
            println!("The result of adding {} and {} is {}", x,y,res);
        }

        Calculator::Sub{x,y} => {
            let res = x - y;
            println!("The result of subtracting {} from {} is {}", y,x,res);
        }

        Calculator::Mul{x,y} =>{
            let res = x*y;
            println!("the result of the multiplication between {} and {} is {}", x,y,res);
        }

        Calculator::Div{x,y} =>{
            let res = x/y;
            println!("The result of dividing {} by {} is {}", x,y,res);
        }

    }
}
