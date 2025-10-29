# Plan for asedia-shell

## Style

Should be done in single colour themed with respect to current system base16 scheme.
Write a stylix module for this.

## Binary - asedia-shell

This is the main interface that will run as a daemon.

### Dashboard

Main interface hovering on desktop.

Divided to a 3-row grid.

(0, 0) contains 3 bars about battery, cpu and memory
(0, 1) contains the clock, with timedate and today events powered by khal
(0, 2) contains my profile photo and screen lock prohibitor

(1, 0) contains a quote from fortune with a refresh button
(1, 1) contains RSS feed from Welkin, miniflux

(2, 1) contains MPD controls
(2, 2) contains weather powered by wego
(2, 3) contains the system tray

### Quickcontrol Panel

Buttons and switchs available when swiped downwards.

Left part is notifications powered by mako and MPRIS controls

Right part contains:
- Volume and brightness sliders
- Screen lock prohibitor button
- Rotation controls
- show-my-osk controls
    Or if possible, fork niri
- Waydroid mode switch

### Dock

Switch and summon applications when swiped upwards.

Lists all currently active workspaces along with active application, their name, icon and workspace id.
Also shows the button to call launchpad.

### Launchpad

An application launcher, lists all possible applications in a grid with a search bar above for fuzzy search.
When clicked on, use the WM to summon the application to avoid child process on asedia-shell.
Will not automatically switch workspaces.

## Binary - asedia-logout

This is executable, will show up when called.

### Logout screen

Five buttons:
- Poweroff
- Reboot
- Suspend
- Lock
- Logout

And a line in the centre that displays either `Asedia` or the action names.
Actions need to be confirmed.

## Binary - asedia-lock

This is executable, will show up when called and runs until unlocked.

### Lock screen

TBC
