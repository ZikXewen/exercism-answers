#!/usr/bin/env bash

echo "$1" | tr '-' ' ' | tr '[:lower:]' '[:upper:]' | sed 's/[^A-Z ]//g' | sed 's/\([A-Z]\)[A-Z]* */\1/g'
