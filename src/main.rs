use std::ops::Index;


fn main() {
   let v1 = vec![1, 2, 3];

   let mut v3 = Vec::new();
   v3.push(1);
   v3.push(2);
   v3.push(3);

   println!("{:?}", v3);

   let x = v3.pop();

   println!("{:?}", x);
   println!("{:?}", v3);

   let y = v3[1];
   let z = v3.get( 100);
   println!("{:?}", z);

   let s = &v3[..1];

}   

