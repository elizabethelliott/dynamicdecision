#!/bin/bash

for i in *_control.mp4; do ffmpeg -i "$i" -c:v libvpx-vp9 -crf 35 -b:v 0 -b:a 128k -c:a libopus -cpu-used -4 -deadline realtime "${i%.mp4}.webm"; done
