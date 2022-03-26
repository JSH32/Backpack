#!/bin/bash
PORT=3001 ./backpack &
(cd ./client && BACKEND_PORT=3001 npm run start -- -p 3000) &
wait -n
exit $?