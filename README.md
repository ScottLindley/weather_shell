# Weather Shell

Fetches the temperature in your current location for use in your shell prompt.

## Usage (only tested with zsh)

1. Obtain an API key from Pirate Weather [here](https://pirateweather.net/getting-started)
2. Add the following to your `zshrc`
  ```sh
  # Weather shell prompt
  export PIRATE_WEATHER_API_KEY="YOUR_API_KEY_HERE"
  export PROMPT="$(weather_shell) $PROMPT"
  ```
3. Install the binary `cargo install --path .`
