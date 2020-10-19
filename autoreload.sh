#!/bin/sh
systemfd --no-pid -s http::3030 -- cargo watch --features "autoreload" -x 'run'
