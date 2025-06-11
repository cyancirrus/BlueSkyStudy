#!/bin/bash

cargo run &
SERVER_PID=$!
start_server() {
	until curl -s -o /dev/null 0.0.0.0:3000; do
		echo "waiting for server to start"
		sleep 0.5
	done
	echo "server started testing endpoints"
}

application_home() {
	echo $(curl 0.0.0.0:3000)
}

start_server
application_home

kill $SERVER_PID
