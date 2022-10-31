/*STACK AND HEAP MEMORY */
/*stack heap and box, box dereference */
fn main() {
    /*
       by default int, float are stored in the stack meomory
       box will allow us to allocate the memory inside heap
       variable also can be defined inside the inner scope
       as soon as the scope is completed the memory is also deallocated
    */

    let box1 = Box::new(5);
    {
        let box2 = Box::new(4);
        /*
            to retrive the value inside the box we have to dereference it by using *
        */
        let sum = *box1 + *box2;
        println!("Sum is {sum}");
    }
    box3();

    let p1 = Point { x: 5.5, y: 6.7 };
    dbg!(&p1);
    drop(p1);
    // dbg!(p1);
}

/*
    we also can allocate the memory inside the function similar to inner scope
    memory allocated by box1 inside the function box3 will also be freed after the
    function call is completed
*/
fn box3() {
    let box1 = Box::new(5);
    println!("The value is {}", box1);
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

/*
    We have a drop trait which is used to drop the memory allocated by the variable
    befor the scope is completed, it is used for the memory cleanups
*/

impl Drop for Point {
    fn drop(&mut self) {
        println!("Dropping poits ({}, {})", self.x, self.y);
    }
}
