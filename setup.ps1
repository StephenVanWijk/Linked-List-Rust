# Create subproject directories and files for a Rust workspace

$projects = @(
    "bad_safe_doubly_linked_deque",
    "bad_singly_linked_stack",
    "ok_singly_linked_stack",
    "ok_unsafe_doubly_linked_deque",
    "persistent_singly_linked_stack",
    "unsafe_singly_linked_queue"
)

foreach ($proj in $projects) {
    $projPath = ".\$proj"
    $srcPath = "$projPath\src"
    New-Item -ItemType Directory -Path $srcPath -Force | Out-Null

    # Create minimal Cargo.toml for each subproject
    $cargoToml = @"
[package]
name = "$proj"
version = "0.1.0"
edition = "2021"

[dependencies]
"@
    Set-Content -Path "$projPath\Cargo.toml" -Value $cargoToml

    # Create minimal src/lib.rs
    $libRs = "// $proj library root"
    Set-Content -Path "$srcPath\lib.rs" -Value $libRs
}