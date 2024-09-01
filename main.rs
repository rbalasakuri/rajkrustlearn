fn main() {
}

struct Node {
 data : i32,
 link : Option<Box<Node>>,
}

struct Stack {
   count :i32,
   top : Option<Box<Node>>,
}


impl Stack {
   
   pub fn creatstack() -> Self {
      let none : Option<Box<Node>> = None;
      Self {
           count : 0,
           top : none,
       }
   }

  pub fn Push (stack :  & mut Stack, x : i32) -> bool 
   {
         let node =  Node { 
            data : x,
            link : stack.top
         };
      
         stack.top = Some(Box::new(node));
         stack.count += 1;
         true
   }

  }











