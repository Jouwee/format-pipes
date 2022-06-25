#!/bin/sh
BASEDIR=$(cd "$(dirname "$0")"; pwd)

echo Creating aliases for: $BASEDIR

chmod +x $BASEDIR/target/debug/format-pipes
alias qrcode=$BASEDIR/target/debug/format-pipes