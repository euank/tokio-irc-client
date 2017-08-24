#!/bin/bash

set -o pipefail
set -eu
set -x

DOCKER_IRCD_NAME="tokio_test_ircd"

tmpdir=$(mktemp -d)
docker rm -f ${DOCKER_IRCD_NAME} 2>/dev/null || true
docker run -d --name ${DOCKER_IRCD_NAME} -p 6667:6667 inspircd/inspircd-docker:2.0.24
#trap "rm -rf ${tmpdir}; docker rm -f ${DOCKER_IRCD_NAME}" EXIT

for i in $(seq 1 10); do
	if nc localhost 6667 -w 1 </dev/null; then
		break
	fi
	if [[ "$i" == 10 ]]; then
		echo "irc did not come up in time"
		exit 1
	fi
	sleep 2
done

sleep 2

export IRC_SERVER="localhost:6667"

./target/debug/examples/print_messages &
# ./target/debug/examples/print_messages > "$tmpdir/recvd" &
print_messages_pid=$!
#trap "kill -9 $print_messages_pid; rm -rf ${tmpdir}; docker rm -f ${DOCKER_IRCD_NAME}" EXIT

# Give it time to join channel
sleep 5

./target/debug/examples/send_message

if ! grep "Hello World" "${tmpdir}/recvd"; then
	echo "Expected contents to contain 'Hello world'; was:"
	cat "$tmpdir/recvd"
	exit 1
fi

if ! grep "Goodbye world" "${tmpdir}/recvd"; then
	echo "Expected contents to contain 'Goodbye world'; was:"
	cat "$tmpdir/recvd"
	exit 1
fi
