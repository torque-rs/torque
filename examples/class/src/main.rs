struct Class {}

#[m8::class]
impl Class {
	#[constructor]
	pub fn new() -> Self {
		Self {}
	}
}

fn main() {
	println!("Hello, world!");
}
