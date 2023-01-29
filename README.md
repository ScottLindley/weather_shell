# Weather Shell

Fetches the temperature in your current location for use in your shell prompt.

## Install

```sh
curl "https://raw.githubusercontent.com/ScottLindley/weather_shell/main/install.sh" | sh
```

## Configure

1. Obtain an API key from Pirate Weather [here](https://pirateweather.net/getting-started)
2. Add the following to your `zshrc`

```sh
# Weather shell prompt
export PIRATE_WEATHER_API_KEY="YOUR_API_KEY_HERE"
export PROMPT="$(weather_shell) $PROMPT"
```
