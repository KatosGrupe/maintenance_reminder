#!/usr/bin/env bash
set -euo pipefail

CONTAINER_NAME=maintenance_reminder:latest

sudo docker image build ./ --tag $CONTAINER_NAME
sudo docker save $CONTAINER_NAME | ssh -C root@kx3n.l.dedikuoti.lt docker load
ssh -C root@kx3n.l.dedikuoti.lt docker run -d $CONTAINER_NAME
