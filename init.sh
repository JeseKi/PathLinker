#!/bin/bash

DESKTOP_FILE="/usr/share/applications/pathlinker.desktop"
APPIMAGE_PATH="/opt/PathLinker/pathlinker.AppImage"

echo "Creating desktop entry..."
cat <<EOF | sudo tee $DESKTOP_FILE > /dev/null
[Desktop Entry]
Name=PathLinker
Type=Application
Exec=$APPIMAGE_PATH %u
NoDisplay=false
MimeType=x-scheme-handler/pathlinker;
Categories=Utility;
StartupNotify=true
StartupWMClass=PathLinker
X-Desktop-File-Install-Version=0.23
Terminal=false
Icon=pathlinker
EOF

sudo desktop-file-install $DESKTOP_FILE
sudo update-desktop-database

echo "Copying AppImage..."
sudo mkdir -p /opt/PathLinker
sudo cp pathlinker* "$APPIMAGE_PATH" || { echo "Copy failed"; exit 1; }
sudo chmod +x "$APPIMAGE_PATH"

echo "Initialization complete. Press enter to continue."
read
