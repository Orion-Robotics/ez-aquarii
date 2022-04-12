#!/usr/bin/env bash

# controller or camera
COMPONENTS=${1:-controller}

if [ "$COMPONENTS" == "camera" ]; then
  jurigged ./camera &
fi
if [ "$COMPONENTS" == "controller" ]; then
  echo "started controller, listening for changes..."
  ./controller/target/controller &
  CONTROLLER_PID=$!
  inotifywait -qme modify,create,delete,move ./controller/target |
  while read -r dir event file; do
    if [ "$file" = "controller" ]; then
      if [ -z "$CONTROLLER_PID" ]; then
        kill -9 CONTROLLER_PID
      fi
      tput setaf 3
      echo "reloading..."
      tput sgr0
      ./controller/target/controller &
      CONTROLLER_PID=$!
    fi
  done
fi
