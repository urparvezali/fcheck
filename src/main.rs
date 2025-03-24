use std::collections::HashSet;

use cargo_metadata::MetadataCommand;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    name: String,
}
fn main() {
    let args = Cli::parse();
    let metadata = MetadataCommand::new()
        .exec()
        .expect("Failed to get Cargo metadata");

    let target_crate = args.name;

    if let Some(pkg) = metadata.packages.iter().find(|p| p.name == target_crate) {
        let all_features: HashSet<_> = pkg.features.keys().collect();
        let enabled_features: HashSet<_> = pkg
            .features
            .keys()
            .filter(|&f| {
                metadata
                    .resolve
                    .as_ref()
                    .unwrap()
                    .nodes
                    .iter()
                    .any(|node| node.id == pkg.id && node.features.contains(f))
            })
            .collect();

        let disabled_features: Vec<_> = all_features.difference(&enabled_features).collect();

        println!("Enabled: {:?}", enabled_features);
        println!("Disabled: {:?}", disabled_features);
    } else {
        println!("Crate '{}' not found", target_crate);
    }
}
