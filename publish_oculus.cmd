@echo off
cargo build --release --target aarch64-linux-android
del %~dp0..\spacefighter-wings\src\SpacefighterWings\Assets\Plugins\Android\aarch64\libintercosmic_empire.so"
copy %~dp0target\aarch64-linux-android\release\libintercosmic_empire.so %~dp0..\spacefighter-wings\src\SpacefighterWings\Assets\Plugins\Android\aarch64