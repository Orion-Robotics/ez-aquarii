#!/bin/bash
#
# Sync local directory to remote directory.
#
# Modified by: Leo Mao
# Modified from: https://gist.github.com/evgenius/6019316
#
# Requires Linux, bash, inotifywait and rsync.
#
# To avoid executing the command multiple times when a sequence of
# events happen, the script waits one second after the change - if
# more changes happen, the timeout is extended by a second again.
#
# Example usage:
#    
#    watchsync.sh . host:/remote/dir
#
# Released to Public Domain. Use it as you like.

EVENTS="CREATE,CLOSE_WRITE,DELETE,MODIFY,MOVED_FROM,MOVED_TO"
RSYNC_OPTIONS="-Cait -I"

if [ -z "$2" ]; then
  echo "Usage: $0 /path/to/localdir [user@]hostname:/path/to/dir"
  exit -1;
fi

localdir=$1
remotedir=$2

run() {
  rsync $RSYNC_OPTIONS $localdir $remotedir
}

run

inotifywait -q -e "$EVENTS" -m -r --format '%:e %f' $1 | (
  WAITING="";
  while true; do
    LINE="";
    read -t 1 LINE;
    if test -z "$LINE"; then
      if test ! -z "$WAITING"; then
        echo "CHANGE";
        WAITING="";
      fi;
    else
      WAITING=1;
    fi;
  done) | (
  while true; do
    read TMP;
    run
  done
)