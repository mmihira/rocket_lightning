#!/bin/bash

parent_dir="$PARENT_DIRECTORY_OF_REPO""/rocket_rc"
cd "${parent_dir}/rc_signal"

context_dir="/tmp/rocket_rc/rc_signal"
rc_context="$context_dir"/ctx
[ -e "$context_dir"  ] && rm -rf "$context_dir"
mkdir -p "$rc_context"
cp -r ./* "$rc_context"
cp -r ../lib "$context_dir"
cp ../docker/rc_signal/Dockerfile "$context_dir"
cp ../docker/rc_signal/deps.sh "$context_dir"
cp ../docker/rc_signal/entrypoint.sh "$context_dir"
cd "$context_dir"
[ -e ./ctx/target ] && rm -rf ./ctx/target


if [ -d /tmp/rc_signal ]; then rm -r /tmp/rc_signal; fi
mkdir -p /tmp/rc_signal
cd "$context_dir"
tar --exclude='./target/' -zcvf "/tmp/rc_signal/rc_signal.gzip" ./
