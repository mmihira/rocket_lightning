docker run \
  --name rc_signal \
  --restart always \
  --log-driver syslog \
  -e RC_POSTGRES_HOST=rocket_rc_pg \
  -e RC_POSTGRES_PASSWORD="$POSTGRES_PASSWORD" \
  --network=rocket_net \
  -d \
  rc_signal

