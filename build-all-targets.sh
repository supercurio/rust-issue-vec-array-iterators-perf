#!/bin/sh

if [ ! -z ${1} ]; then
	vec_resize="--features vec_resize"
fi

build() {
	if [ ! -z ${1} ]; then
		target="--target=$1"
		echo "\nBuild target $1\n"
	fi
			
	cargo build $vec_resize --release $target
}

build

build arm-unknown-linux-gnueabihf
build armv7-unknown-linux-gnueabihf

build armv7-linux-androideabi
build aarch64-linux-android

