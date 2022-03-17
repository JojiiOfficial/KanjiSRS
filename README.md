# KanjiSRS
SRS tool to learn kanji. It uses [jotoba](https://jotoba.de) to display kanji/stroke information.
You can use it to review kanji you want to learn by drawing them.

# Usage
```
USAGE:
    kanji_srs [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -h, --help                         Print this help information
        --max-reviews <max-reviews>    Max amount of reviews (Defaut 20)
        --new-count                    Specify how many new cards you want to learn (Default = 8)
        --no-new                       Don't add new kanji, just review old ones

SUBCOMMANDS:
    add       Adds kanji to learn
    remove    Removes kanji from database
    reset     Reset learn process of a kanji and treat it as a new item
    review    Manually tag kanji as reviewed
    info      Show info about reviews
    stats     Show stats
    fix-db    Fix database
    help      Print this message or the help of the given subcommand(s)
```
```
Note: To start a review run it without any arguments
```

# Adding new Kanji

- `kanji_srs add <TEXT>`
- `echo <TEXT> | kanji_srs add -`

`<TEXT>` can contain kanji and non-kanji. Non Kanji and already added Kanji are skipped

# Stats

You can view your kanji statistics with the `stats` subcommand:<br>
```
╭──────────────────────╮
│      Kanji stats     │
├──────────────┬───────┤
│ Total Kanji  │ 108字 │
├──────────────┼───────┤
│ In learning  │ 16字  │
├──────────────┼───────┤
│ Percentage   │ 14.8% │
├──────────────┼───────┤
│ Days left    │ 12日  │
╰──────────────┴───────╯
```

# Examples
```shell
kanji_srs add 今日は天気が悪い # Adds 今日天気悪 unless they're already existing
```

```shell
kanji_srs # Starts a new review + learning session.
```

```shell
kanji_srs --no-new # Starts a review session without learning new kanji
```

```shell
kanji_srs fix-db # Tries to repair a broken database
```
