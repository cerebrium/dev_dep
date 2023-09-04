use Parse::parse::Dependencies;

mod Parse;

fn main() {
    let mut file_parser = Dependencies::default();
    file_parser.parse_deps_to_hashmap("package.json");
    println!(
        "Dependencies: {:?}, \n \n \n \n dev dependencies: {:?}",
        file_parser.dependencies, file_parser.devDependencies
    );
}
