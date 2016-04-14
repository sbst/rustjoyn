extern crate rustjoyn;

fn main() {
	println!("basic_client");
	rustjoyn::public_function();
	rustjoyn::indirect_access();
}
