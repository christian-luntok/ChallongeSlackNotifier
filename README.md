# Challonge (Table Tennis) Slack Notifier

### Written in Rust 🦀

Forked from https://github.com/joshgermon/ChallongeSlackNotifier

Challonge is a very popular website to track casual and professional sports competitions.

While they have an API, they don't unfortunately have any kind of outgoing webhooks to inform Slack or any other service when a match is recorded.

Challonge's API is also very questionably structured (particularly when it relates to finding the latest match). You cannot receive only the completed matches of a competition you can only get back all of them and manually filter through checking "null" properties.

All this for just wanting to post your scores in Slack?

That's where this small application handles that all for you. Just run it on a server or your computer in the background and while it's running it will watch for new matches and post it to your Slack channel.

## What does this do?

This is an application that will run and continuously poll a Challonge Tournament for completed matches.

Once it detects a completed match it will push a notification to a chosen Slack channel.

### Currently this is formatted for Table Tennis competitions only. This could be updated in the future to support further options.

## How do I use this?

Either clone down this repository locally and run `cargo build --release` or simply download the precomplied release from this repo. _*Coming soon!*_

#### ⚠️ Right now there is no way to run this without editing the hardcoded endpoints. Work is currently being made to provide command line args or a config file to run this script.
