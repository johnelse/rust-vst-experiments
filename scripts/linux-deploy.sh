#!/bin/bash

TARGET=/usr/local/lib/vst

if [ ! -d target/release ]
then
    echo "No release build found!"
    echo "Run 'cargo build --release', then run this script from the repo root."
    exit 1
fi

mkdir -p $TARGET
cp -f target/release/*.so $TARGET
