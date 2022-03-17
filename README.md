# KanjiSRS
SRS tool to learn kanji


# Usage
```shell
USAGE:
    kanji_srs [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -h, --help                         Print help information
        --max-reviews <max-reviews>    Max amount of reviews
        --new-count                    Specify how many new cards you want to learn
        --no-new                       Don't add new kanji, just review old

SUBCOMMANDS:
    add       Adds kanji to learn
    remove    Removes kanji from database
    help      Print this message or the help of the given subcommand(s)
    info      Show info about reviews
    reset     Reset learn process of a kanji and treat it as a new item
    review    Manually tag kanji as reviewed
    stats     Show stats
    fix-db    Fix database
```
```
Note: To start a review run it without any arguments
```
