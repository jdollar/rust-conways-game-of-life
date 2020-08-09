#!/usr/bin/env sh

DIR="$(dirname "$0")"
DEBUG_DIR="$DIR/target/debug"
RELEASE_DIR="$DIR/target/release"

if cargo "$@"; then
  if [ -d $DEBUG_DIR ]; then
    cp -r "${DIR}/"{config,assets} $DEBUG_DIR
    tar -czvf $DEBUG_DIR/debug-conway-release.tar.gz -C $DEBUG_DIR {assets,config,rust-conways-game-of-life}
  fi

  if [ -d $RELEASE_DIR ]; then
    cp -r "${DIR}/"{config,assets} $RELEASE_DIR
    tar -czvf $RELEASE_DIR/conway-release.tar.gz -C $RELEASE_DIR {assets,config,rust-conways-game-of-life}
  fi
fi
