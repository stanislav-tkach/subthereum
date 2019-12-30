use vergen::{generate_cargo_keys, ConstantsFlags};

const ERROR_MSG: &str = "Failed to generate metadata files";

fn main() {
    generate_cargo_keys(ConstantsFlags::SHA_SHORT).expect(ERROR_MSG);
    println!("cargo:rerun-if-changed=.git/HEAD");
}
