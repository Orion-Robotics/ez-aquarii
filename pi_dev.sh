#!/usr/bin/env bash

# controller or camera
COMPONENTS=${1:-controller}

if [ "$COMPONENTS" == "camera" ]; then
  jurigged ./camera &
fi
if [ "$COMPONENTS" == "controller" ]; then
  inotifywait -rqme modify,create,delete,move ./controller/target/controller |
  while read -r filename event; do
    tput setaf 3
    echo "reloading..."
    tput sgr0
    ./controller/target/controller
  done
fi
