# Patanos

Note system for the fish shell

## How to install

Run `cargo install --locked --git git@github.com:BenSampaolo/Patanos.git` to install.

## How to use

Patanos or **Pa**th **ta**sk **no**te **s**ystem is used for taking notes on usability or to track task across directorys and most impoertantly with a reference of the directory the entry was made in.

For making a not use the `--note` or `-n` flag followed by your own message

Tasks can be saved using the `--task` or `-t` flag followed by your own message

The `--list` or `-l` flag lists all current entry's, sorted alphabetically and with an index. Optionally you can write a number as an argument to only print out a specified number of entries.

To remove a note or a task use the `--remove` or `-r` flag with the index aquired from the `--list` flag

Using multiple flags at once is possible but not recommended

## Future plans

- Notes and Tasks are basically the same at the moment. In the future Notes will appear at the bottom when listed, and tasks can be marked as done instead of deleting them
- Timestamp's should be saved to every note and task as another way of sorting for the printout
