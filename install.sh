curl -L "https://github.com/ScottLindley/weather_shell/releases/latest/download/$(uname -m)" > weather_shell
BIN_DIR=/usr/local/bin
sudo mv weather_shell $BIN_DIR/weather_shell
sudo chown $USER $BIN_DIR/weather_shell
chmod +x $BIN_DIR/weather_shell
