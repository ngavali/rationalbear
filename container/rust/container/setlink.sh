#!/bin/bash
PID=$1
ip link add name brahma type bridge
ip addr add 192.168.1.10/24 brd + dev brahma
ip link add veth0 type veth peer name veth1 netns ${PID}
ip link set veth0 master brahma
ip link set brahma up
ip link set veth0 up
