mod quad;

fn main() {
  let q_pi = quad::stoq("3.14159265358979323846264338327950288419716939937510");
  println!("{:?}", q_pi);
  println!("{:?}", quad::qtos(q_pi));
  let q_eulergamma = quad::stoq("0.57721566490153286060651209008240243104215933593992");
  println!("{:?}", q_eulergamma);
  let q_ln2 = quad::stoq("0.69314718055994530941723212145817656807");
  println!("{:?}", q_ln2);

  let x = quad::Quad::new(1.0,0.0); 
  let y = quad::Quad::new(0.0,0.1); 
  println!("{:?}", x);
  println!("{:?}", y);
  println!("{:?}", quad::Quad::new(1.0,0.1));
  println!("{:?}", (x+y)*(x+y));
  println!("{:?}", (x*y)+(x*y));
  println!("{:?}", (x+y)*10.0);
  println!("{:?}", quad::Quad::new(1.0,0.0)/10.0);
  println!("{:?}", (quad::Quad::new(1.0,0.0)/10.0)*10.0);
  println!("{:?}", quad::Quad::new(1.0,0.1).scale2(3));
  let mut z = quad::Quad::new(1.0,0.0);
  z /= 10.0;
  println!("{:?}", z);
  println!("{:?}", quad::qtos(z));
  println!("{:?}", quad::qtos(quad::Quad::new(0.1,0.0)));
}
