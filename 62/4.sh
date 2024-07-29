#!/bin/bash

menu() {
    clear
    echo "1. cargo new [Create a new Rust project]"
    echo "2. *cargo build [Build the project]"
    echo "3. *cargo build --release [Build the project in release mode]"
    echo "4. *cargo run [Run the project]"
    echo "5. *cargo test [Run the tests]"
    echo "6. *cargo bench [Run the benchmarks]"
    echo "7. *cargo check [Check the project for errors]"
    echo "8. cargo clean [Remove the target directory]"
    echo "9. *cargo update [Update dependencies]"
    echo "10. *cargo doc [Build this project's and its dependencies' documentation]"
    echo "11. *cargo publish [Package and upload this project to crates.io]"
    echo "12. *cargo install [Install a Rust binary]"
    echo "13. *cargo fmt [Format the project's code]"
    echo "14. *cargo clippy [Lint the project's code]"
    echo "15. *cargo search [Search for crates in crates.io]"
    echo "16. *cargo version [Show version information]"
    echo "17. *cargo metadata [Output project dependency information]"
    echo "18. *cargo release [Automate the release process of a Rust project]"
    echo "19. rustc build [Compile with rustc]"
    echo "20. rustc run [Compile and run with rustc]"
    echo "21. rustc --version [Show rustc version information]"
    echo "22. rustc --explain [Explain a rustc error code]"
    echo "23. rustc --test [Build a test harness]"
    echo "24. rustc -W help [Show all possible warnings]"
    echo "25. rustc -C help [Show all possible codegen options]"
    echo "26. rustc --print [Print compiler information]"
    echo "27. Custom Command [Run a custom cargo or rustc command]"
    echo "x. Exit [Exit the script]"

    read -p "Enter a command: " choice

    case "$choice" in
        1) cargo_new ;;
        2) cargo_build ;;
        3) cargo_build_release ;;
        4) cargo_run ;;
        5) cargo_test ;;
        6) cargo_bench ;;
        7) cargo_check ;;
        8) cargo_clean ;;
        9) cargo_update ;;
        10) cargo_doc ;;
        11) cargo_publish ;;
        12) cargo_install ;;
        13) cargo_fmt ;;
        14) cargo_clippy ;;
        15) cargo_search ;;
        16) cargo_version ;;
        17) cargo_metadata ;;
        18) cargo_release ;;
        19) rustc_build ;;
        20) rustc_run ;;
        21) rustc_version ;;
        22) rustc_explain ;;
        23) rustc_test ;;
        24) rustc_warnings ;;
        25) rustc_codegen ;;
        26) rustc_print ;;
        27) custom_command ;;
        [Xx]) exit ;;
        *) echo "Invalid choice. Press any key to return to the menu." ;;
    esac
    read -n 1 -s -r -p "Press any key to continue..."
    menu
}

cargo_new() {
    clear
    read -p "Enter a project name: " project_name
    echo "Running cargo new..."
    cargo new "$project_name"
}

cargo_build() {
    clear
    echo "Running cargo clean and then cargo build..."
    cargo clean
    cargo build
}

cargo_build_release() {
    clear
    echo "Running cargo clean and then cargo build --release..."
    cargo clean
    cargo build --release
}

cargo_run() {
    clear
    echo "Running cargo clean and then cargo run..."
    cargo clean
    cargo run
}

cargo_test() {
    clear
    echo "Running cargo clean and then cargo test..."
    cargo clean
    cargo test
}

cargo_bench() {
    clear
    echo "Running cargo clean and then cargo bench..."
    cargo clean
    cargo bench
}

cargo_check() {
    clear
    echo "Running cargo clean and then cargo check..."
    cargo clean
    cargo check
}

cargo_clean() {
    clear
    echo "Running cargo clean..."
    cargo clean
}

cargo_update() {
    clear
    echo "Running cargo clean and then cargo update..."
    cargo clean
    cargo update
}

cargo_doc() {
    clear
    echo "Running cargo clean and then cargo doc..."
    cargo clean
    cargo doc
}

cargo_publish() {
    clear
    echo "Running cargo clean and then cargo publish..."
    cargo clean
    cargo publish
}

cargo_install() {
    clear
    echo "Running cargo clean and then cargo install..."
    cargo clean
    cargo install
}

cargo_fmt() {
    clear
    echo "Running cargo clean and then cargo fmt..."
    cargo clean
    cargo fmt
}

cargo_clippy() {
    clear
    echo "Running cargo clean and then cargo clippy..."
    cargo clean
    cargo clippy
}

cargo_search() {
    clear
    echo "Running cargo clean and then cargo search..."
    cargo clean
    cargo search
}

cargo_version() {
    clear
    echo "Running cargo clean and then cargo --version..."
    cargo clean
    cargo --version
}

cargo_metadata() {
    clear
    echo "Running cargo clean and then cargo metadata..."
    cargo clean
    cargo metadata
}

cargo_release() {
    clear
    echo "Running cargo clean and then cargo release..."
    cargo clean
    cargo release
}

rustc_build() {
    clear
    echo "Running rustc build..."
    rustc main.rs
}

rustc_run() {
    clear
    echo "Running rustc run..."
    rustc main.rs -o main.exe
    ./main.exe
}

rustc_version() {
    clear
    echo "Running rustc --version..."
    rustc --version
}

rustc_explain() {
    clear
    read -p "Enter a rustc error code: " error_code
    echo "Running rustc --explain $error_code..."
    rustc --explain "$error_code"
}

rustc_test() {
    clear
    echo "Running rustc --test..."
    rustc --test main.rs
}

rustc_warnings() {
    clear
    echo "Running rustc -W help..."
    rustc -W help
}

rustc_codegen() {
    clear
    echo "Running rustc -C help..."
    rustc -C help
}

rustc_print() {
    clear
    echo "Running rustc --print..."
    rustc --print
}

custom_command() {
    clear
    read -p "Enter a custom cargo or rustc command: " custom_command
    echo "Running $custom_command..."
    $custom_command
}

menu  # Start the menu
