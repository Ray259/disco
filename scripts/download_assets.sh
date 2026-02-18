#!/bin/bash

# Define the target directory and file name
AUDIO_DIR="public/audio"
AUDIO_FILE="tiger_king.mp3"
TARGET_AUDIO="$AUDIO_DIR/$AUDIO_FILE"
AUDIO_URL="https://lambda.vgmtreasurechest.com/soundtracks/disco-elysium-official-soundtrack-2019/zoyywpja/04%20Tiger%20King.mp3"

IMAGE_DIR="public/images"
IMAGE_FILE="library_hero.jpg"
TARGET_IMAGE="$IMAGE_DIR/$IMAGE_FILE"
IMAGE_URL="https://shared.akamai.steamstatic.com/store_item_assets/steam/apps/632470/library_hero.jpg"

# Create target directories
mkdir -p "$AUDIO_DIR"
mkdir -p "$IMAGE_DIR"

# Download Audio
if [ -f "$TARGET_AUDIO" ]; then
    echo "Audio file already exists at $TARGET_AUDIO. Skipping download."
else
    echo "Downloading audio file to $TARGET_AUDIO..."
    curl -s -L "$AUDIO_URL" -o "$TARGET_AUDIO"
    if [ $? -eq 0 ]; then
        echo "Audio download successful."
    else
        echo "Failed to download the audio file."
    fi
fi

# Download Image
if [ -f "$TARGET_IMAGE" ]; then
    echo "Image file already exists at $TARGET_IMAGE. Skipping download."
else
    echo "Downloading image file to $TARGET_IMAGE..."
    curl -s -L "$IMAGE_URL" -o "$TARGET_IMAGE"
    if [ $? -eq 0 ]; then
        echo "Image download successful."
    else
        echo "Failed to download the image file."
    fi
fi
