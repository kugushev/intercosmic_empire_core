@echo off
cargo build
cargo test --test bindings
echo Don't forget to RUN HOT RELOAD!!!
pause