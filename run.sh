#!/usr/bin/env sh
#Env variables here!
LOG_FILE="log"
BIN_FILE="target/debug/main"
BOT_PID_FILE="bot_pid"

export TOKEN="<Your token here>"

export OWNERS="<owner1>,<owner2>..."
export LOG_CHANNEL="<Your channel id here. if dont needed, you can not specify>"

export TEMP_FILES_PATH="path/to/your/tmp"


#Bot start
./$BIN_FILE >$LOG_FILE &

#Write pid to file
echo $! >$BOT_PID_FILE