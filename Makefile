ARCH = x64
RM = del

build-installer:
	cargo build --release
	wix build yy-battery-notifier-rs.wxs -arch ${ARCH} -ext WixToolset.UI.wixext -o yy-battery-notifier-rs_${ARCH}.msi

clean:
	${RM} yy-battery-notifier-rs_*.wixpdb
	${RM} yy-battery-notifier-rs_*.msi

re arch:clean build-installer
