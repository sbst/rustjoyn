extern crate rustjoyn;

fn main() {
	println!("basic_service");
	rustjoyn::public_function();
	rustjoyn::indirect_access();
}
