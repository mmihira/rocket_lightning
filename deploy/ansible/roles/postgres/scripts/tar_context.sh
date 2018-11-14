#!/bin/bash

context="$PARENT_DIRECTORY_OF_REPO""/rocket_rc"
if [ -d /tmp/rocket_rc_deploy ]; then rm -r /tmp/rocket_rc_deploy; fi
mkdir -p /tmp/rocket_rc_deploy
cd "${context}/docker"
tar -zcvf "/tmp/rocket_rc_deploy/pg.gzip" ./postgres

