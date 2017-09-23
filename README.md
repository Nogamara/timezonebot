# TimezoneBot - because Time Zones are hard

## why?

This bot will parse some time formats on Discord and reply with a few conversions.

Examples:

```
starting at 9:30 cet
9:30 CET is 00:09 PST / 02:09 CST / 03:09 EST

starting at 20:30 msk
20:30 MSK is 10:09 PST / 12:09 CST / 13:09 EST / 19:09 CET

starting at 9pm pst
9:00PM PST is 23:09 CST / 00:09 EST / 06:09 CET
```

## howto

```
cargo build --release
export DISCORD_TOKEN=...
./target/release/timezonebot
```

To add the bot to your discord:
```
https://discordapp.com/oauth2/authorize?client_id=361085311079546880&scope=bot
```
Open in your browser and click through.

## stuff

  * uses [serenity](https://github.com/zeyla/serenity/) for the heavy lifting
  * bot structure ripped from [serenity](https://github.com/zeyla/serenity/tree/master/examples/01_basic_ping_bot)
