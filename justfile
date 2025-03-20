set shell := ["cmd.exe", "/c"]

alias b:= build-installer

default: help

help:
    echo "Usage: just <recipe>"
    echo ""
    just --list

build-installer arch:
    cargo build --release
    cargo build --profile release-debug-assertions
    wix build yy-battery-notifier-rs.wxs -arch {{arch}} -ext WixToolset.UI.wixext -o yy-battery-notifier-rs_{{arch}}.msi

clean:
    del yy-battery-notifier-rs_*.wixpdb
    del yy-battery-notifier-rs_*.msi

re arch:clean (build-installer arch)
