#! /usr/bin/sh

ls /tmp/*.log | sort | tail -n 1 | xargs tail -f
