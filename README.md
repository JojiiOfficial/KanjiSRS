# KanjiSRS
SRS tool to learn kanji


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
