#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: $0 hostname"
    exit -1;
fi

./watchsync.sh ./camera pi@$1:/home/pi/ez-aquarii
