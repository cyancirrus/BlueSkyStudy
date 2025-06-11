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
	RESPONSE=$(curl -s \
		http://0.0.0.0:3000
	)
	echo "$RESPONSE"
	WELCOME=$( echo "$RESPONSE" | jq -r '.msg' )
	echo "$WELCOME"
	if [[ "$WELCOME" == "Welcome to AutumnSky!" ]]; then
		echo "✅ Success"
	else
		echo "❌ Unexpected response: $RESPONSE"
		kill $SERVER_PID
		exit 1
	fi
}

# request_subscribe() {
# 	RESPONSE=$(curl -s --request POST http://0.0.0.0:3000/subscribe)
# 	if [[ "$RESPONSE" == "Async i am subscribing" ]]; then
# 		echo "✅ Success"
# 	else
# 		echo "❌ Unexpected response: $RESPONSE"
# 		kill $SERVER_PID
# 		exit 1
# 	fi
# }

request_newsfeed() {
	RESPONSE=$(curl -s -X POST \
		-H "Content-Type: application/json" \
		-d '{"user":1}' \
		http://0.0.0.0:3000/newsfeed
	)
	STATUS=$(echo "$RESPONSE" | jq -r '.status')
	FEED_LEN=$(echo "$RESPONSE" | jq '.feed | length')
	if [[ "$STATUS" == "ok" && "$FEED_LEN" -gt 0 ]]; then
		echo "✅ Success"
	else
		echo "❌ Unexpected response: $RESPONSE"
		kill $SERVER_PID
		exit 1
	fi
}


start_server
application_home
# request_subscribe
request_newsfeed

kill $SERVER_PID
