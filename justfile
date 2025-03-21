set shell := ["cmd.exe", "/c"]

alias b:= build-installer

default: help

help:
    echo "Usage: just <recipe>"
    echo ""
    just --list

build-installer arch:
    cargo build --release --features gui
    copy target\release\yy-battery-notifier-rs.exe target\release\yy-battery-notifier-rs_gui.exe
    cargo build --release
    wix build yy-battery-notifier-rs.wxs -arch {{arch}} -ext WixToolset.UI.wixext -o yy-battery-notifier-rs_{{arch}}.msi

clean:
    del yy-battery-notifier-rs_*.wixpdb
    del yy-battery-notifier-rs_*.msi

re arch:clean (build-installer arch)
