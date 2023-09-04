use serde::Deserialize;
use serde_json::{Result, Value};
use std::fs::read_to_string;

#[derive(Deserialize, Default, Debug)]
pub struct Dependencies {
    pub dependencies: Value,
    pub devDependencies: Value,
}

impl Dependencies {
    pub fn parse_deps_to_hashmap(&mut self, path: &str) -> Result<()> {
        let fp = read_to_string(path).expect("cannot find path to package.json");

        let parsed_json: Value = serde_json::from_str(&fp)?;
        let deps: Dependencies =
            serde_json::from_value(parsed_json).expect("error turning serde into struct");

        self.dependencies = deps.dependencies;
        self.devDependencies = deps.devDependencies;
        Ok(())
    }
}
