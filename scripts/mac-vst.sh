#!/bin/bash

# Run this script to create loadable bundles for macOS. Output will be in the
# mac-vst directory.

if [ ! -d target/release ]
then
    echo "No release build found!"
    echo "Run 'cargo build --release', then run this script from the repo root."
    exit 1
fi

for FILE in `ls target/release/*.dylib`
do
    LIBNAME=`basename $FILE .dylib`
    NAME=${LIBNAME:3}
    ROOT=mac-vst/$NAME.vst/Contents

    mkdir -p $ROOT/MacOS
    cp $FILE $ROOT/MacOS/$NAME
    sed -e "s/@NAME@/$NAME/g" scripts/Info.plist > $ROOT/Info.plist
done
