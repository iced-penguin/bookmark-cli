# bookmark-cli

A command-line tool for managing bookmarks of directories. 

## Overview

This tool aims to allow users to save paths to specific directories as bookmarks, making it easy to access them later. 
Users can add, search, list, and delete bookmarks.

Bookmarks are stored in a hidden file (`~/.bookmarks`).

![demo](https://github.com/user-attachments/assets/c5a5f7ad-ce47-42a9-b5c9-b946c5db06b0)

## Requirements

- `rustc` and `cargo`

## Installation

```
cargo install bookmark-cli
```

## Usage

| Command | Description |
| --- | --- |
| add | Add a bookmark |
| delete | Delete a bookmark |
| search | Search for a bookmark |
| list | List bookmarks |
| prune| Delete all broken bookmarks |
| help | Print this message or the help of the given subcommand(s) |

### Adding a Bookmark

```
bm add [bookmark path]
```

The bookmark path must be an absolute path.

If you do not specify `[bookmark path]`, the current directory will be registered as a bookmark.

### Searching for a Bookmark

```
bm search
```

A search prompt will appear, allowing you to filter and select a bookmark.

### Listing Saved Bookmarks

```
bm list
```

All bookmarks will be displayed.

### Deleting a Bookmark

```
bm delete
```

A prompt will appear for you to select the bookmark you want to delete.

### Deleting Broken Bookmarks

```
bm prune
```

All broken bookmarks, i.e. bookmarks that no longer exist, will be deleted.

## Moving to a Bookmark

You can move to a selected bookmark by combining it with the `cd` command.

If you are using zsh, you can add a function to your `~/.zshrc`. 

For example:

```sh
function cb() {
  local -r dir=$(bm search)
  if [ -z "$dir" ]; then
    return 1
  fi
  cd "$dir" || return 1
}
```

