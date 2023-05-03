#!/bin/bash

ffmpeg -i "${1}.webm" -ss $2 -c:v libvpx-vp9 -crf 35 -b:v 0 -b:a 128k -c:a libopus -cpu-used 4 -deadline realtime "${1}_trimmed.webm"
