# Installation
Download the latest release and extract the file to your preferred location.  
Example: `~/.local/share/wallpapers`

# Configuration
Create file `~/.config/wallpapers/settings` containing the following
```
Image folder=~/Pictures
Duration=1800
Override color scheme=prefer-light
```

# Autostart
If you don't have a easy way to add commands to autostart you can add the follow file `~/.config/autostart/Wallpapers.desktop`
```
[Desktop Entry]
Type=Application
Name=Wallpapers
Description=Starts wallpapers app
Exec=/preferred/installation/location/wallpapers run
```

# Commands
|Command|Description|
|-|-|
|run|Start wallpaper slideshow, changes images every time timer runs out|
|next|Changes wallpaper to a random image|
