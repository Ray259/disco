#!/bin/bash

# Define the target directory and file name
TARGET_DIR="public/audio"
FILE_NAME="tiger_king.mp3"
TARGET_FILE="$TARGET_DIR/$FILE_NAME"
AUDIO_URL="https://lambda.vgmtreasurechest.com/soundtracks/disco-elysium-official-soundtrack-2019/zoyywpja/04%20Tiger%20King.mp3"

# Create the target directory if it doesn't exist
mkdir -p "$TARGET_DIR"

# Check if the file already exists
if [ -f "$TARGET_FILE" ]; then
    echo "Audio file already exists at $TARGET_FILE. Skipping download."
else
    echo "Downloading audio file to $TARGET_FILE..."
    curl -s -L "$AUDIO_URL" -o "$TARGET_FILE"
    
    if [ $? -eq 0 ]; then
        echo "Download successful."
    else
        echo "Failed to download the audio file."
        exit 1
    fi
fi
