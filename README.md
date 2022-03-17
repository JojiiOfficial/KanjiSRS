# KanjiSRS
SRS tool to learn kanji. It uses [jotoba](https://jotoba.de) to display kanji/stroke information.
You can use it to review kanji you want to learn by drawing them.

# Purpose
Learning kanji is hard, especially recalling stroke order and remembering them. The most effective way for me was to write kanji 
again and again by hand. Not one by one, 100 times each, but rather shuffling a few ones around. This way you don't write the kanji you want to learn barely thinking about it several times but 
have to actually recall the stroke order again and again each time since you just wrote another kanji. This is pretty hard to do with Anki since it sequentially shows you your kanji
and puts them away after you recall them. With this tool you can see all the kanji you're learning at once, and thus you're able to pick a few of them and shuffle them around.

# Bugs
- Adding/removing/reviewing while another instance is running (eg. waiting for review to be done) gets overwritten when that other instance exits

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
