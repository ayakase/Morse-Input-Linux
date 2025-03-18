#!/bin/bash

# Build the Rust project in release mode
cargo build --release

# Move the binary to /usr/local/bin
sudo mv target/release/morse_input /usr/local/bin/

# Create a .desktop file
cat <<EOF | tee ~/.local/share/applications/morse_input.desktop > /dev/null
[Desktop Entry]
Version=1.0
Type=Application
Name=Morse Input
Comment=A Morse code input tool
Exec=/usr/local/bin/morse_input
Icon=morse_input
Terminal=false
Categories=Utility;
EOF

# Make the .desktop file executable
chmod +x ~/.local/share/applications/morse_input.desktop

# Refresh the desktop menu
xdg-desktop-menu forceupdate

echo "Setup complete! You can now run Morse Input from the app menu."
