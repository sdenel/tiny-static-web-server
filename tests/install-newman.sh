#!/usr/bin/env bash
NODE_VERSION=v10.8.0
NODE_NAME="node-$NODE_VERSION-linux-x64"

mkdir -p "$PWD/bin/"
cd "$PWD/bin/"

if [ ! -d "$NODE_NAME" ]; then
	wget -nc https://nodejs.org/dist/$NODE_VERSION/$NODE_NAME.tar.xz
	tar Jxvf $NODE_NAME.tar.xz
	rm $NODE_NAME.tar.xz
fi
export PATH=$PWD/$NODE_NAME/bin:$PATH
npm install
npm install newman
export PATH=$PWD/node_modules/@angular/cli/bin:$PATH