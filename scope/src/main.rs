fn main() {
    //scope is the boundaries or region of code where a name is valid
    // Block is the area between an opening curly brace ad a closing curly brace
        let coffre_price  =5.99; 

        { 
            let cookie_price = 2.99;
            println!("The price of the cookie is ${}", cookie_price);
            println!("The price of the coffee is ${}", coffre_price);
            //cookie_price is valid here only is valid inside this block.
        }
          println!("The price of the cookie is ${}", coffre_price);
          //valid outside the block
}