#!/bin/bash

PI_HOST=$1
BUILD_TYPE=${2:-debug}
PI_USER=${3:-pi}

CONTROLLER_PATH=./controller/target/armv7-unknown-linux-gnueabihf/$BUILD_TYPE

tput setaf 5
echo -e "Watching $CONTROLLER_PATH for changes..."
tput sgr0

inotifywait -rqme modify,create,delete,move $CONTROLLER_PATH |
while read -r filename event; do
  tput setaf 3
  echo "uploading binary..."
  tput sgr0
  rsync -avz $CONTROLLER_PATH/controller $PI_USER@$PI_HOST:/home/$PI_USER/ez-aquarii/controller/target
done
