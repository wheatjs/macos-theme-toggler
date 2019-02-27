# MacOS Theme Toggler

Toggles Mojave's Theme based on the time of day. Feel free to contribue to the project. This is my first real rust project, so the code quality may not be up to standards.

## Running the software
### Building
First you must build it before you can use it. This will change in the future, but for now as long as you have cargo installed just run
```bash
cargo build --release
```
After it builds the binary will be in `macos_theme_toggler/target/macos_theme_toggler` from there you can just run the program with the following command
```bash
./macos_theme_toggler/target/macos_theme_toggler [latitude: float] [longitude: float]
```

# Roadmap
* Add a way to run it at startup
* Maybe put it on homebrew so it is easier to install
* Maybe add a way to auto configure the lat and long based on the users IP, maybe use seomthing like https://ipinfo.io/json since they do not require an API key.
* Clean up the code a bit I guess ¯\\\_(ツ)_/¯