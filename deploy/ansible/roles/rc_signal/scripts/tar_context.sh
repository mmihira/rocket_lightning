#!/bin/bash

if [ -d /tmp/docker_rocket_rc ]; then rm -r /tmp/docker_rocket_rc; fi
mkdir -p /tmp/docker_rocket_rc
cd "${PARENT_DIRECTORY_OF_REPO}/rc_signal"
tar --exclude='./target/' -zcvf "/tmp/docker_rocket_rc/rc_signal.gzip" ./

