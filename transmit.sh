#!/usr/bin/env bash

BB_IP=192.168.0.175
BB_PASSWD=msoe
ROBOT_DIR=/home/rmc/robot

rsync -P --rsh="sshpass -p $BB_PASSWD ssh -l rmc -o StrictHostKeyChecking=no" target/debug/rmc-core rmc@${BB_IP}:${ROBOT_DIR}/debug
rsync -P --rsh="sshpass -p $BB_PASSWD ssh -l rmc -o StrictHostKeyChecking=no" target/release/rmc-core rmc@${BB_IP}:${ROBOT_DIR}/release
rsync -P --rsh="sshpass -p $BB_PASSWD ssh -l rmc -o StrictHostKeyChecking=no" enable-pwm.sh rmc@${BB_IP}:${ROBOT_DIR}/enable-pwm.sh
rsync -P --rsh="sshpass -p $BB_PASSWD ssh -l rmc -o StrictHostKeyChecking=no" Rocket.toml rmc@${BB_IP}:${ROBOT_DIR}/Rocket.toml
rsync -P --rsh="sshpass -p $BB_PASSWD ssh -l rmc -o StrictHostKeyChecking=no" -r ./static/ rmc@${BB_IP}:${ROBOT_DIR}/static/

rsync -P --rsh="sshpass -p $BB_PASSWD ssh -l rmc -o StrictHostKeyChecking=no" -r ./test-release/ rmc@${BB_IP}:${ROBOT_DIR}/test-release/
rsync -P --rsh="sshpass -p $BB_PASSWD ssh -l rmc -o StrictHostKeyChecking=no" -r ./test-debug/ rmc@${BB_IP}:${ROBOT_DIR}/test-debug/