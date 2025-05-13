# Arach-NO! bot

A small inside joke Discord bot
initially created to take jabs at people reposting from the
[r/DestinyTheGame](https://www.reddit.com/r/DestinyTheGame/) subreddit.

## Description

A Discord bot written in Rust using the serenity/poise libraries.
Initially just reacted with an emoji to posts matching a subreddit regex.

Later upgraded with a connection to Firebase Realtime Database which is used to track and store
the number of times someone reacted with a "haha" emoji,
indicating that they thought the message was humorous.
The haha functionality is accompanied with two slash commands:

| Slash command  | Output                                          |
| -------------- | ----------------------------------------------- |
| `/hahas list`  | Shows a leaderboard with the top 10 hahas users |
| `/hahas count` | Shows your personal hahas count                 |

Ultimately evolved to also track and delete Twitter links to avoid sending them traffic.

## Getting Started

### Dependencies

Rust toolchain that supports edition 2021 or Docker

### Installing

- Install the rust stable toolchain
- Clone the repository
- `cargo build --release`

The output is a single binary which can be run from anywhere.

Alternatively, choose a target to build from the available Dockerfiles:

- [armv7](./Dockerfile-armv7-unknown-linux-musleabihf) (runs on Raspberry Pi)
- [x86_64](./Dockerfile-x86_64-unknown-linux-musl) (runs on x86 linux machines)

and run `docker build . --file Dockerfile=<my_target_dockerfile>`

### Executing program

To run the bot, the following environment variables are required:

| Environment variable                                                                                                                                                  | Example                                                                                   |
| --------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- |
| `DISCORD_TOKEN` Your app's **Client secret** in the [Discord developer portal](https://discord.com/developers/applications)                                           | `Bot CNEfXkJHryAN69i5oMU5j`<br>`D0j5IgwG71WkdE2HIZ9LB4Nrh`<br>`TQXZ8yXuR5lt79156DKbW6r31` |
| `FIREBASE_URL` The **URL** of your Firebase Realtime Database from the Realtime Database tab in the [Firebase console](https://console.firebase.google.com)           | `https://my-firebase-app.firebaseio.com`                                                  |
| `FIREBASE_TOKEN` Your Firebase Realtime Database's **apiKey** from the Project Settings > General page in the [Firebase console](https://console.firebase.google.com) | `trluVm-o0Ua5gSkzF606dMSO9vcvakIg6vUnzRJX`                                                |

Don't try to use the example values - they are randomly generated.

## Version History

0.3.3 No longer reacts to twitter links with a funny emoji.
Now deletes them with extreme prejudice

0.3.2 Fixed an issue where links beginning with "www" were wrongly flagged

0.3.1 Now covers fx/vx twitter fixers

0.3.0 - Now reacts to twitter links with a funny emoji

0.2.4 - Updated dependencies

## License

This project is licensed under the MIT License -
see the [LICENSE](./LICENSE) file for details
