#!/usr/bin/env bash
set -euo pipefail

HOST=gedimai.acm.lt
CONTAINER_NAME=maintenance_reminder:latest

sudo docker image build ./ --tag $CONTAINER_NAME
sudo docker save $CONTAINER_NAME | ssh -C root@$HOST docker load
ssh -C root@$HOST 'docker stop $(docker ps -q)' || true
ssh -C root@$HOST docker run -d -p 443:8000 $CONTAINER_NAME
