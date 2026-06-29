#!/usr/bin/env bash

tmux new-session -d -s "dbus-rs"
tmux rename-window "make"
tmux new-window -n "src"
tmux send-keys -t "dbus-rs" "nvim" Enter
tmux select-window -t "make"
tmux attach-session -t "dbus-rs"
