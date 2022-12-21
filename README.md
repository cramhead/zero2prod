Watch
`cargo watch -x check -x test -x run`

Run health check with
`curl -v http://127.0.0.1:8000/health_check`

Test
`cargo test`

Lint
`cargo clippy`

Format
`cargo fmt`

Audit packages
`cargo audit`

Measure test coverage
`cargo tarpaulin --ignore-tests --output-dir coverage --target-dir coverage`

Driving the subscription API
`curl -i -X POST -d 'email=thomas_mann@hotmail.com&name=Tom' http://127.0.0.1:8000/subscribe`

Check the subscriptions table
`psql -d newsletter -h 127.0.0.1 -U postgres --password -c "select * from subscriptions"`