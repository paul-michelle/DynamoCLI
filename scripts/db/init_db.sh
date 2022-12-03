#! /bin/bash

set +x
set -e
set -o pipefail


[ -x "$(which docker)" ]                                    || (echo "docker not found" && exit 1)
[ -x "$(which aws)" ]                                       || (echo "aws cli binary not found" && exit 1)
test -f ${INIT_TABLE_DESCRIPTION:="scripts/db/table.json"} || (echo "table description not found at $INIT_TABLE_DESCRIPTION" && exit 1)


if [[ -z "${SKIP_DOCKER}" ]]
then
  docker run -d -p ${DB_URL:="127.0.0.1:8000"}:8000 --name ${CONTAINER_NAME:="dynamodb_local_dev"} amazon/dynamodb-local
fi

ENDPOINT_URL="http://$DB_URL"

until aws dynamodb list-tables --endpoint-url $ENDPOINT_URL
do
  >&2 echo "DinamoDB instance is not available just yet..." && sleep 1
done
>&2 echo "DinamoDB is up and running at $ENDPOINT_URL"

aws dynamodb create-table --cli-input-json "file://$INIT_TABLE_DESCRIPTION" --endpoint-url $ENDPOINT_URL --region hometown
echo "Successfully created table described in $INIT_TABLE_DESCRIPTION"