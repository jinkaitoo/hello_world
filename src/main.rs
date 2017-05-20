fn main() {
    let mut x: i32 = 1;
    x = 7;
    let x = x; // `x` is now immutable and is bound to `7`.
	println!("{}",x);
    let y = 4;
    let y = "I can also be bound to text!"; // `y` is now of a different type.
	println!("{}",y);
	
	if x == 7 {
		println!("x ist {}", x);
	}
}
