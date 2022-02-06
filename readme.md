# Configuration values
Create file `.wallpapers` in home folder containing the following
```
Image folder=/path/to/images
Duration=1800
```

# Autostart
If you don't have a easy way to app commands to autostart you can add the follow file `~/.config/autostart/Wallpapers.desktop`
```
[Desktop Entry]
Type=Application
Name=Wallpapers
Description=Starts wallpapers app
Exec=/path/to/bin/wallpapers run
```

# Commands
|Command|Description|
|-|-|
|run|Start wallpaper slideshow, changes images every time timer runs out|
|next|Changes wallpaper to a random image|
