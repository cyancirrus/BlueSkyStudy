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
	RESPONSE=$(curl -s http://0.0.0.0:3000)
	if [[ "$RESPONSE" == "hello world" ]]; then
		echo "✅ Success"
	else
		echo "❌ Unexpected response: $RESPONSE"
		kill $SERVER_PID
		exit 1
	fi

}

start_server
application_home

kill $SERVER_PID
