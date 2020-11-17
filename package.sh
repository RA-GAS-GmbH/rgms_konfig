#!/bin/bash
# set -e # exit on error

function build() {
	# Run this script two times, one for i686 (32Bit) and for the x86_64 (64Bit)
	for ARCH in i686 x86_64; do
	    export PKG_CONFIG_ALLOW_CROSS=1
	    export PKG_CONFIG_PATH=/usr/${ARCH}-w64-mingw32/sys-root/mingw/lib/pkgconfig/
	    export GTK_INSTALL_PATH=/usr/${ARCH}-w64-mingw32/sys-root/mingw/
	    # build package
	    source ~/.cargo/env
	    cargo build --target=${ARCH}-pc-windows-gnu --release ${CARGO_FEATURES}
	    # extract package name and version from cargo
	    export NAME=$(cargo pkgid | cut -d# -f2 | cut -d: -f1)${NAME_SUFFIX}
	    export VERSION=$(cargo pkgid | cut -d# -f2 | cut -d: -f2)
	    export NAME_VERSION="${NAME}-${VERSION}"
	    # create destination directory
	    mkdir -p "${NAME_VERSION}-windows-${ARCH}"
	    cp target/${ARCH}-pc-windows-gnu/release/*.exe "${NAME_VERSION}-windows-${ARCH}"
	    # extract all dependencies to libs
	    export DLLS=`peldd "${NAME_VERSION}-windows-${ARCH}"/*.exe -t --ignore-errors`
	    for DLL in $DLLS
	    do cp "$DLL" "${NAME_VERSION}-windows-${ARCH}"
	    done
	    # copy the gtk and additional files like the LICENSE and the README
	    mkdir -p "${NAME_VERSION}-windows-${ARCH}"/share/{themes,gtk-3.0}
	    mkdir -p "${NAME_VERSION}-windows-${ARCH}"/resources
	    cp -r $GTK_INSTALL_PATH/share/glib-2.0/schemas "${NAME_VERSION}-windows-${ARCH}"/share/glib-2.0
	    cp -r $GTK_INSTALL_PATH/share/icons "${NAME_VERSION}-windows-${ARCH}"/share/icons
	    # [ -d resources ] && cp -r resources "${NAME_VERSION}-windows-${ARCH}"/
	    [ -d resources ] && cp resources/about.png "${NAME_VERSION}-windows-${ARCH}"/resources/
	    [ -d resources ] && cp resources/Hilfe*.pdf "${NAME_VERSION}-windows-${ARCH}"/resources/
	    [ -d resources ] && cp resources/*.css "${NAME_VERSION}-windows-${ARCH}"/resources/
	    [ -d resources ] && cp resources/*.ico "${NAME_VERSION}-windows-${ARCH}"/resources/
	    [ -d resources ] && cp resources/*.csv "${NAME_VERSION}-windows-${ARCH}"/resources/
		# If NAME_SUFFIX is set (e.g. -ra-gas) we pack the 'internal' version. So add the Beschreibungen.
		if [ ! -z "${NAME_SUFFIX}" ]; then
			[ -d resources ] && cp resources/*"_Beschreibung_RA-GAS Sensor-MB.pdf" "${NAME_VERSION}-windows-${ARCH}"/resources/
		fi
	    [ -d share ] && cp -r share "${NAME_VERSION}-windows-${ARCH}"/
	    [ -f README.md ] && cp -r README.md "${NAME_VERSION}-windows-${ARCH}"/
	    [ -f LICENSE ] && cp -r LICENSE "${NAME_VERSION}-windows-${ARCH}"/
	    # reduce the binary size
	    mingw-strip "${NAME_VERSION}-windows-${ARCH}"/*
	    # zip the whole package dir
	    zip -qr "${NAME_VERSION}-windows-${ARCH}".zip "${NAME_VERSION}-windows-${ARCH}"/*
	    # Make windows installer if Setup.nsi files exist
	    # See: https://gitlab.com/RA-GAS-GmbH/ne4_konfig/-/blob/master/Setup.nsi
	    [ -f Setup${NAME_SUFFIX}.nsi ] && ARCH=$ARCH makensis Setup${NAME_SUFFIX}.nsi
	done
}


echo -e "\n\n>>>>>>>>>>>>>> Kunden Version! <<<<<<<<<<<<<<<<<<\n\n"
NAME_SUFFIX=""
CARGO_FEATURES=""
build


echo -e "\n\n>>>>>>>>>>>>>> RA-GAS Version! <<<<<<<<<<<<<<<<<<\n\n"
NAME_SUFFIX="-ra-gas"
CARGO_FEATURES="--features=ra-gas"
build


