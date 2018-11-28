docker run \
  --name rc_server \
  --restart always \
  --log-driver syslog \
  --log-opt tag=rc_server \
  -e RC_POSTGRES_HOST=rocket_rc_pg \
  -e RC_POSTGRES_PASSWORD="$POSTGRES_PASSWORD" \
  -p 8000:8000 \
  -p 3012:3012 \
  --network=rocket_net \
  -d \
  rc_server

