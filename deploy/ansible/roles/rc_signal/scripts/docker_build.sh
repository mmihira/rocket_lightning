#!/bin/bash

# Currently the docker image is too build to actually do it this way
docker build ./ -t rc_signal
docker save rc_signal /tmp/rc_signal.tar
