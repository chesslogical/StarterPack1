
@echo off

:menu
cls
echo 1. cargo new [Create a new Rust project]
echo 2. *cargo build [Build the project]
echo 3. *cargo build --release [Build the project in release mode]
echo 4. *cargo run [Run the project]
echo 5. *cargo test [Run the tests]
echo 6. *cargo bench [Run the benchmarks]
echo 7. *cargo check [Check the project for errors]
echo 8. cargo clean [Remove the target directory]
echo 9. *cargo update [Update dependencies]
echo 10. *cargo doc [Build this project's and its dependencies' documentation]
echo 11. *cargo publish [Package and upload this project to crates.io]
echo 12. *cargo install [Install a Rust binary]
echo 13. *cargo fmt [Format the project's code]
echo 14. *cargo clippy [Lint the project's code]
echo 15. *cargo search [Search for crates in crates.io]
echo 16. *cargo version [Show version information]
echo 17. *cargo metadata [Output project dependency information]
echo 18. *cargo release [Automate the release process of a Rust project]
echo 19. rustc build [Compile with rustc]
echo 20. rustc run [Compile and run with rustc]
echo 21. rustc --version [Show rustc version information]
echo 22. rustc --explain [Explain a rustc error code]
echo 23. rustc --test [Build a test harness]
echo 24. rustc -W help [Show all possible warnings]
echo 25. rustc -C help [Show all possible codegen options]
echo 26. rustc --print [Print compiler information]
echo 27. Custom Command [Run a custom cargo or rustc command]
echo x. Exit [Exit the script]

set /p choice=Enter a command:

if "%choice%"=="1" goto cargo_new
if "%choice%"=="2" goto cargo_build
if "%choice%"=="3" goto cargo_build_release
if "%choice%"=="4" goto cargo_run
if "%choice%"=="5" goto cargo_test
if "%choice%"=="6" goto cargo_bench
if "%choice%"=="7" goto cargo_check
if "%choice%"=="8" goto cargo_clean
if "%choice%"=="9" goto cargo_update
if "%choice%"=="10" goto cargo_doc
if "%choice%"=="11" goto cargo_publish
if "%choice%"=="12" goto cargo_install
if "%choice%"=="13" goto cargo_fmt
if "%choice%"=="14" goto cargo_clippy
if "%choice%"=="15" goto cargo_search
if "%choice%"=="16" goto cargo_version
if "%choice%"=="17" goto cargo_metadata
if "%choice%"=="18" goto cargo_release
if "%choice%"=="19" goto rustc_build
if "%choice%"=="20" goto rustc_run
if "%choice%"=="21" goto rustc_version
if "%choice%"=="22" goto rustc_explain
if "%choice%"=="23" goto rustc_test
if "%choice%"=="24" goto rustc_warnings
if "%choice%"=="25" goto rustc_codegen
if "%choice%"=="26" goto rustc_print
if "%choice%"=="27" goto custom_command
if /i "%choice%"=="x" goto exit

echo Invalid choice. Press any key to return to the menu.
pause >nul
goto menu

:cargo_new
cls
set /p project_name=Enter a project name:
echo Running cargo new...
cargo new %project_name%
goto return_to_menu

:cargo_build
cls
echo Running cargo clean and then cargo build...
cargo clean
cargo build
goto return_to_menu

:cargo_build_release
cls
echo Running cargo clean and then cargo build --release...
cargo clean
cargo build --release
goto return_to_menu

:cargo_run
cls
echo Running cargo clean and then cargo run...
cargo clean
cargo run
goto return_to_menu

:cargo_test
cls
echo Running cargo clean and then cargo test...
cargo clean
cargo test
goto return_to_menu

:cargo_bench
cls
echo Running cargo clean and then cargo bench...
cargo clean
cargo bench
goto return_to_menu

:cargo_check
cls
echo Running cargo clean and then cargo check...
cargo clean
cargo check
goto return_to_menu

:cargo_clean
cls
echo Running cargo clean...
cargo clean
goto return_to_menu

:cargo_update
cls
echo Running cargo clean and then cargo update...
cargo clean
cargo update
goto return_to_menu

:cargo_doc
cls
echo Running cargo clean and then cargo doc...
cargo clean
cargo doc
goto return_to_menu

:cargo_publish
cls
echo Running cargo clean and then cargo publish...
cargo clean
cargo publish
goto return_to_menu

:cargo_install
cls
echo Running cargo clean and then cargo install...
cargo clean
cargo install
goto return_to_menu

:cargo_fmt
cls
echo Running cargo clean and then cargo fmt...
cargo clean
cargo fmt
goto return_to_menu

:cargo_clippy
cls
echo Running cargo clean and then cargo clippy...
cargo clean
cargo clippy
goto return_to_menu

:cargo_search
cls
echo Running cargo clean and then cargo search...
cargo clean
cargo search
goto return_to_menu

:cargo_version
cls
echo Running cargo clean and then cargo --version...
cargo clean
cargo --version
goto return_to_menu

:cargo_metadata
cls
echo Running cargo clean and then cargo metadata...
cargo clean
cargo metadata
goto return_to_menu

:cargo_release
cls
echo Running cargo clean and then cargo release...
cargo clean
cargo release
goto return_to_menu

:rustc_build
cls
echo Running rustc build...
rustc main.rs
goto return_to_menu

:rustc_run
cls
echo Running rustc run...
rustc main.rs -o main.exe
.\main.exe
goto return_to_menu

:rustc_version
cls
echo Running rustc --version...
rustc --version
goto return_to_menu

:rustc_explain
cls
set /p error_code=Enter a rustc error code:
echo Running rustc --explain %error_code%...
rustc --explain %error_code%
goto return_to_menu

:rustc_test
cls
echo Running rustc --test...
rustc --test main.rs
goto return_to_menu

:rustc_warnings
cls
echo Running rustc -W help...
rustc -W help
goto return_to_menu

:rustc_codegen
cls
echo Running rustc -C help...
rustc -C help
goto return_to_menu

:rustc_print
cls
echo Running rustc --print...
rustc --print
goto return_to_menu

:custom_command
cls
set /p custom_command=Enter a custom cargo or rustc command:
echo Running %custom_command%...
%custom_command%
goto return_to_menu

:return_to_menu
pause
goto menu

:exit
exit
