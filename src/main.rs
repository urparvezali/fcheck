use cargo_metadata::{Metadata, MetadataCommand, Package};
use std::{collections::HashSet, env::args};

fn main() {
    let args = args().collect::<Vec<String>>();

    let metadata = fetch_metadata().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });
    for i in 1..args.len() {
        match find_package(&metadata, &args[i]) {
            Some(pkg) => print_features(&metadata, pkg),
            None => eprintln!("Crate '{}' not found", args[i]),
        }
    }
}

/// Fetches metadata using Cargo's metadata command
fn fetch_metadata() -> Result<Metadata, Box<dyn std::error::Error>> {
    Ok(MetadataCommand::new().exec()?)
}

/// Finds the specified package in the metadata
///
/// # Arguments
/// * `metadata` - Reference to the Cargo metadata
/// * `name` - Name of the package to find
///
/// # Returns
/// * An optional reference to the found package
fn find_package<'a>(metadata: &'a Metadata, name: &str) -> Option<&'a Package> {
    metadata.packages.iter().find(|pkg| pkg.name == name)
}

/// Prints the enabled and disabled features of a package
///
/// # Arguments
/// * `metadata` - Reference to the Cargo metadata
/// * `pkg` - Reference to the package whose features are analyzed
// fn print_features(metadata: &Metadata, pkg: &Package) {
//     let all_features: HashSet<_> = pkg.features.keys().collect();
//     let enabled_features: HashSet<_> = pkg
//         .features
//         .keys()
//         .filter(|&f| {
//             metadata.resolve.as_ref().map_or(false, |resolve| {
//                 resolve
//                     .nodes
//                     .iter()
//                     .any(|node| node.id == pkg.id && node.features.contains(f))
//             })
//         })
//         .collect();

//     let disabled_features: Vec<_> = all_features
//         .difference(&enabled_features)
//         .cloned()
//         .collect();
//     let mut enabled_features: Vec<_> = enabled_features.into_iter().collect();

//     enabled_features.sort_unstable();
//     let mut disabled_features = disabled_features;
//     disabled_features.sort_unstable();
//     println!("{}:", pkg.name);
//     println!("\tEnabled: {:?}", enabled_features);
//     println!("\tDisabled: {:?}", disabled_features);
// }
fn print_features(metadata: &Metadata, pkg: &Package) {
    let all_features: HashSet<_> = pkg.features.keys().collect();
    let enabled_features: HashSet<_> = pkg
        .features
        .keys()
        .filter(|&f| {
            metadata.resolve.as_ref().map_or(false, |resolve| {
                resolve
                    .nodes
                    .iter()
                    .any(|node| node.id == pkg.id && node.features.contains(f))
            })
        })
        .collect();

    let disabled_features: Vec<_> = all_features
        .difference(&enabled_features)
        .cloned()
        .collect();
    let mut enabled_features: Vec<_> = enabled_features.into_iter().collect();

    enabled_features.sort_unstable();
    let mut disabled_features = disabled_features;
    disabled_features.sort_unstable();

    println!("{}:", pkg.name);
    println!("├── Enabled:");
    for feature in &enabled_features {
        println!("│   ├── {}", feature);
    }
    if enabled_features.is_empty() {
        println!("│   └── (none)");
    }

    println!("└── Disabled:");
    for feature in &disabled_features {
        println!("    ├── {}", feature);
    }
    if disabled_features.is_empty() {
        println!("    └── (none)");
    }
}
