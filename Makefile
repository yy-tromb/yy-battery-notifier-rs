ARCH = x64
RM = del
CP = copy

build-installer:
	cargo build --release --features gui
	${CP} target\release\yy-battery-notifier-rs.exe target\release\yy-battery-notifier-rs_gui.exe
	cargo build --release
	wix build yy-battery-notifier-rs.wxs -arch ${ARCH} -ext WixToolset.UI.wixext -o yy-battery-notifier-rs_${ARCH}.msi

clean:
	${RM} yy-battery-notifier-rs_*.wixpdb
	${RM} yy-battery-notifier-rs_*.msi

re arch:clean build-installer
