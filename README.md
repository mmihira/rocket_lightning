Install cargo-make

cd ./docker/
docker-compose up
cd ../rc_signal
diesel setup

Don't run tests in parallel ! Anything involving the database. DOn't run tests in parallel.

cargo test -- --test-threads=1
