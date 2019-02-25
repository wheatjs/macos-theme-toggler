# MacOS Theme Toggler

Toggles Mojave's Theme based on the time of day. Feel free to contribue to the project. This is my first real rust project, so the code quality may not be up to standards.

## Configuration
Your latitude and longitude are required to determine when sunset and sunrise are. You can either configure it here, or when you run the progarm it will ask you to configrue it there if it hasn't already been configured here.
```toml
#Your Latitude
LAT=10.0

#Your Longitude
LONG=10.0
```

# Roadmap
* Add a way to run it at startup
* Maybe put it on homebrew so it is easier to install
* Maybe add a way to auto configure the lat and long based on the users IP, maybe use seomthing like https://ipinfo.io/json since they do not require an API key.
* Clean up the code a bit I guess ¯\\\_(ツ)_/¯