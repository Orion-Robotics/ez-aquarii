#!/bin/bash

PI_USER=${2:-pi}

inotifywait -q -m -e modify,create,delete -r ./camera |
while read -r filename event; do
  tput setaf 3
  echo "detected camera changes, uploading..."
  tput sgr0
  rsync -avz ./camera $PI_USER@$1:/home/$PI_USER/ez-aquarii/camera 
done
