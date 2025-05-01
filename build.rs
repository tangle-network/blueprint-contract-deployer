use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

#[allow(clippy::too_many_lines, clippy::format_push_string)]
fn main() {
    if env::var("CARGO_FEATURE_BUILD_SCRIPT").is_err() {
        return;
    }

    let contract_dirs = vec![
        "./dependencies/eigenlayer-middleware-0.5.4/lib/eigenlayer-contracts",
        "./dependencies/eigenlayer-middleware-0.5.4",
        "./contracts",
    ];
    soldeer_install();
    soldeer_update();
    build_contracts(contract_dirs);

    // Create bindings directory
    let src_dir = Path::new("src");
    let bindings_dir = src_dir.join("bindings");

    // Remove existing bindings directory if it exists
    if bindings_dir.exists() {
        fs::remove_dir_all(&bindings_dir).unwrap();
    }
    fs::create_dir_all(&bindings_dir).unwrap();

    // Define contracts from contracts directory
    let contracts_contracts = ["SquaringTask", "SquaringServiceManager"];

    // Define contracts from eigenlayer-middleware directory
    let middleware_contracts = [
        "IAllocationManager",
        "AllocationManager",
        "AVSDirectory",
        "BLSApkRegistry",
        "DelegationManager",
        "EigenPod",
        "EigenPodManager",
        "EmptyContract",
        "ISlashingRegistryCoordinator",
        "IndexRegistry",
        "InstantSlasher",
        "OperatorStateRetriever",
        "PauserRegistry",
        "ProxyAdmin",
        "PermissionController",
        "RegistryCoordinator",
        "RewardsCoordinator",
        "IServiceManager",
        "SlashingRegistryCoordinator",
        "SocketRegistry",
        "StakeRegistry",
        "StrategyBase",
        "StrategyFactory",
        "StrategyManager",
        "TransparentUpgradeableProxy",
        "UpgradeableBeacon",
        "IStrategy",
    ];

    // Generate bindings for contracts directory
    println!("Generating bindings for contracts...");

    // Build the command with all the select flags
    let mut cmd = Command::new("forge");
    cmd.args([
        "bind",
        "--alloy",
        "--skip-build",
        "--evm-version",
        "shanghai",
        "--bindings-path",
        "src/bindings/deploy",
        "--overwrite",
        "--root",
        "./contracts",
        "--module",
    ]);

    // Add select flags for each contract
    for contract in &contracts_contracts {
        cmd.args(["--select", &format!("^{}$", contract)]);
    }

    let status = cmd
        .status()
        .expect("Failed to execute forge bind command for contracts");

    assert!(status.success());

    // Generate bindings for middleware directory
    println!("Generating bindings for middleware...");

    // Build the command with all the select flags
    let mut cmd = Command::new("forge");
    cmd.args([
        "bind",
        "--alloy",
        "--skip-build",
        "--evm-version",
        "shanghai",
        "--bindings-path",
        "src/bindings/core",
        "--overwrite",
        "--root",
        "./dependencies/eigenlayer-middleware-0.5.4",
        "--module",
    ]);

    // Add select flags for each contract
    for contract in &middleware_contracts {
        cmd.args(["--select", &format!("^{}$", contract)]);
    }

    let status = cmd
        .status()
        .expect("Failed to execute forge bind command for middleware");

    assert!(status.success());

    // Post-process the generated files to add the required imports
    println!("Post-processing generated files...");

    // Process deploy contracts
    for contract in &contracts_contracts {
        let lower_contract = contract.to_lowercase();
        let file_path = format!("src/bindings/deploy/{}.rs", lower_contract);
        add_imports_to_file(&file_path, contract);
    }

    // Process middleware contracts
    for contract in &middleware_contracts {
        let lower_contract = contract.to_lowercase();
        let file_path = format!("src/bindings/core/{}.rs", lower_contract);
        add_imports_to_file(&file_path, contract);

        if *contract == "AllocationManager" || *contract == "IAllocationManager" {
            let path = Path::new(&file_path);
            let mut file = fs::File::open(path).unwrap_or_else(|_| panic!("Failed to modify {}", file_path));
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .unwrap_or_else(|_| panic!("Failed to read {}", file_path));

            let new_contents = contents.replace(
                "#[derive(Clone)]
    pub struct AllocateParams {",
                "#[derive(Clone, Hash, Debug, Eq, PartialEq)]\n\tpub struct AllocateParams {",
            );

            fs::write(path, new_contents).expect(&format!("Failed to write to {}", file_path));
        }
    }

    // Create the mod.rs in the bindings directory
    let mut contents = String::new();
    contents.push_str("pub mod core;\n");
    contents.push_str("pub mod deploy;\n");
    contents.push('\n');

    for contract in &contracts_contracts {
        let lower_contract = contract.to_lowercase();
        contents.push_str(&format!(
            "pub use deploy::{}::{};\n",
            lower_contract, contract
        ));
    }
    for contract in &middleware_contracts {
        let lower_contract = contract.to_lowercase();
        contents.push_str(&format!(
            "pub use core::{}::{};\n",
            lower_contract, contract
        ));
    }

    let path = Path::new("src/bindings/mod.rs");
    fs::write(path, contents).expect("Failed to write to mod.rs");

    // Create core/mod.rs to re-export OperatorSet
    let mut core_mod_contents = String::new();
    core_mod_contents.push_str("// This file is generated by the build script\n");
    core_mod_contents.push_str("// Do not edit manually\n\n");

    // Add all modules
    for contract in &middleware_contracts {
        let lower_contract = contract.to_lowercase();
        core_mod_contents.push_str(&format!("pub mod {};\n", lower_contract));
    }

    // Re-export OperatorSet from AllocationManager
    core_mod_contents.push_str("\n// Re-export OperatorSet for use across modules\n");
    core_mod_contents
        .push_str("pub use self::allocationmanager::AllocationManager::OperatorSet;\n");

    let core_mod_path = Path::new("src/bindings/core/mod.rs");
    fs::write(core_mod_path, core_mod_contents).expect("Failed to write to core/mod.rs");

    // Create deploy/mod.rs
    let mut deploy_mod_contents = String::new();
    deploy_mod_contents.push_str("// This file is generated by the build script\n");
    deploy_mod_contents.push_str("// Do not edit manually\n\n");

    // Add all modules
    for contract in &contracts_contracts {
        let lower_contract = contract.to_lowercase();
        deploy_mod_contents.push_str(&format!("pub mod {};\n", lower_contract));
    }

    // Import OperatorSet from core
    deploy_mod_contents.push_str("\n// Import OperatorSet from core\n");
    deploy_mod_contents.push_str("pub use crate::bindings::core::OperatorSet;\n");

    let deploy_mod_path = Path::new("src/bindings/deploy/mod.rs");
    fs::write(deploy_mod_path, deploy_mod_contents).expect("Failed to write to deploy/mod.rs");
}

fn add_imports_to_file(file_path: &str, contract: &str) {
    // Read the file
    let path = Path::new(file_path);
    if !path.exists() {
        println!("Warning: File {} does not exist", file_path);
        return;
    }

    let mut file = fs::File::open(path).unwrap_or_else(|_| panic!("Failed to open {}", file_path));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .unwrap_or_else(|_| panic!("Failed to read {}", file_path));

    // Add the imports at the top
    let new_contents = format!(
        "#![allow(clippy::all, clippy::pedantic, clippy::nursery, warnings, unknown_lints, rustdoc::all, elided_lifetimes_in_paths)]\nuse {}::*;\n\n{}",
        contract, contents
    );

    // Write back to the file
    let mut file =
        fs::File::create(path).unwrap_or_else(|_| panic!("Failed to create {}", file_path));
    file.write_all(new_contents.as_bytes())
        .unwrap_or_else(|_| panic!("Failed to write to {}", file_path));
}

/// Build the Smart contracts at the specified directories.
///
/// This function will automatically rerun the build if changes are detected in the `src`
/// directory within any of the directories specified. Due to this, it is recommended to
/// ensure that you only pass in directories that contain the `src` directory and won't be
/// modified by anything else in the build script (otherwise, the build will always rerun).
///
/// # Panics
///
/// - If the Cargo Manifest directory is not found.
/// - If the `forge` executable is not found.
/// - If the `foundry.toml` file is not found in any of the specified directories
pub fn build_contracts(contract_dirs: Vec<&str>) {
    // Get the project root directory
    let root = workspace_or_manifest_dir();

    // Try to find the `forge` executable dynamically
    let forge_executable = find_forge_executable();

    for dir in contract_dirs {
        let full_path = root.join(dir).canonicalize().unwrap_or_else(|_| {
            println!(
                "Directory not found or inaccessible: {}",
                root.join(dir).display()
            );
            root.join(dir)
        });

        if full_path.exists() {
            if full_path != root.join("./contracts") {
                // Check if foundry.toml exists and add evm_version if needed
                let foundry_toml_path = full_path.join("foundry.toml");

                // We need to pin the evm_version of each foundry.toml with the same version so contracts are all consistent
                if foundry_toml_path.exists() {
                    // Read the existing foundry.toml
                    let mut content = String::new();
                    std::fs::File::open(&foundry_toml_path)
                        .expect("Failed to open foundry.toml")
                        .read_to_string(&mut content)
                        .expect("Failed to read foundry.toml");

                    // Only add evm_version if it's not already there
                    if !content.contains("evm_version") {
                        // Find the [profile.default] section
                        if let Some(pos) = content.find("[profile.default]") {
                            // Insert evm_version after the section header
                            let mut new_content = content.clone();
                            let insert_pos = content[pos..]
                                .find('\n')
                                .map_or(content.len(), |p| p + pos + 1);
                            new_content.insert_str(insert_pos, "    evm_version = \"shanghai\"\n");

                            // Write the modified content back
                            std::fs::write(&foundry_toml_path, new_content)
                                .expect("Failed to write to foundry.toml");
                        } else {
                            // If [profile.default] section doesn't exist, append it
                            let mut file = std::fs::OpenOptions::new()
                                .append(true)
                                .open(&foundry_toml_path)
                                .expect("Failed to open foundry.toml for appending");

                            file.write_all(b"\n[profile.default]\nevm_version = \"shanghai\"\n")
                                .expect("Failed to append to foundry.toml");
                        }
                    }
                } else {
                    panic!("Failed to read dependency foundry.toml");
                }
            }

            // Run forge build with explicit EVM version
            let status = Command::new(&forge_executable)
                .current_dir(&full_path)
                .arg("build")
                .arg("--evm-version")
                .arg("shanghai")
                .arg("--use")
                .arg("0.8.27")
                .status()
                .expect("Failed to execute Forge build");

            assert!(
                status.success(),
                "Forge build failed for directory: {}",
                full_path.display()
            );
        } else {
            panic!(
                "Directory not found or does not exist: {}",
                full_path.display()
            );
        }
    }
}

fn is_directory_empty(path: &Path) -> bool {
    fs::read_dir(path)
        .map(|mut i| i.next().is_none())
        .unwrap_or(true)
}

fn workspace_or_manifest_dir() -> PathBuf {
    let dir = env::var("CARGO_WORKSPACE_DIR")
        .or_else(|_| env::var("CARGO_MANIFEST_DIR"))
        .expect("neither CARGO_WORKSPACE_DIR nor CARGO_MANIFEST_DIR is set");
    PathBuf::from(dir)
}

/// Run soldeer's 'install' command if the dependencies directory exists and is not empty.
///
/// # Panics
/// - If the Cargo Manifest directory is not found.
/// - If the `forge` executable is not found.
/// - If forge's `soldeer` is not installed.
pub fn soldeer_install() {
    // Get the project root directory
    let root = workspace_or_manifest_dir();

    // Check if the dependencies directory exists and is not empty
    let dependencies_dir = root.join("dependencies");
    if !dependencies_dir.exists() || is_directory_empty(&dependencies_dir) {
        let forge_executable = find_forge_executable();

        println!("Populating dependencies directory");
        let status = Command::new(&forge_executable)
            .current_dir(&root)
            .args(["soldeer", "install"])
            .status()
            .expect("Failed to execute 'forge soldeer install'");

        assert!(status.success(), "'forge soldeer install' failed");
    } else {
        println!("Dependencies directory exists or is not empty. Skipping soldeer install.");
    }
}

/// Run soldeer's `update` command to populate the `dependencies` directory.
///
/// # Panics
/// - If the Cargo Manifest directory is not found.
/// - If the `forge` executable is not found.
/// - If forge's `soldeer` is not installed.
pub fn soldeer_update() {
    // Get the project root directory
    let root = workspace_or_manifest_dir();

    // Try to find the `forge` executable dynamically
    let forge_executable = find_forge_executable();

    let status = Command::new(&forge_executable)
        .current_dir(&root)
        .args(["soldeer", "update", "-d"])
        .status()
        .expect("Failed to execute 'forge soldeer update'");

    assert!(status.success(), "'forge soldeer update' failed");
}

/// Returns a string with the path to the `forge` executable.
///
/// # Panics
/// - If the `forge` executable is not found i.e., if Foundry is not installed.
#[must_use]
pub fn find_forge_executable() -> String {
    // Try to find the `forge` executable dynamically
    match Command::new("which").arg("forge").output() {
        Ok(output) => {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            assert!(
                !path.is_empty(),
                "Forge executable not found. Make sure Foundry is installed."
            );
            path
        }
        Err(e) => panic!("Failed to find `forge` executable: {e}"),
    }
}
