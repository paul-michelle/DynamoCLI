#! /bin/bash

CONTAINER_NAME=${CONTAINER_NAME:="dynamodb_local_dev"}

docker ps -aq --filter "name=$CONTAINER_NAME" | grep -q . \
    && (docker stop $CONTAINER_NAME && docker rm $CONTAINER_NAME && echo "Container found and removed") \
    || echo "Container not found"