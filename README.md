# doc-sync

A CLI to convert rust documentation to markdown files and then back to rust documentation. Useful if you want to use a proper markdown editor for editing documentation comments or if you want to
organize your crate in something like an Obsidian Canvas.

doc-sync is currently in a very early stage and is mainly made for my use cases. Therefore, there are some things it does not support. See [Limitations](#limitations) for more info, but overall expect
some things to cause errors. doc-sync shouldn't make any destructive changes though; if it does, please make a GitHub Issue! (However, this doesn't mean you shouldn't be cautious and `git commit` or
`dura capture` before running `from-markdown`)

In the future, we plan to allow using doc-sync as a library, but for now it is only available for usage as an executable.

## Installation and Usage

```sh
cargo install doc-sync
```

After installation, the `doc-sync` executable should be installed to your `PATH`. Please use `doc-sync --help` and `doc-sync [SUBCOMMAND] --help` for info on available options.

Convert rust documentation to markdown files (by default, they will be outputted to `./target/doc-sync`):

```sh
doc-sync to-markdown
```

Convert markdown files to rust documentation:

```sh
doc-sync from-markdown
```

## Limitations

Here is an incomplete list of situations doc-sync currently doesn't support but may support in the future:

-   Don't expect inner items such as struct fields to be supported very well. It's hard to do this when we depend on rustdoc because we have to translate between rustdoc JSON types and syn. When we
    stop depending on rustdoc this should be fixed
-   Inner items that aren't recognized by rustdoc such as functions in functions
-   Duplicate items such as those created with `#[cfg]` and `#[cfg(not)]`. Currently, the first matching item should be used, probably in the order they appear in the file, but I haven't tested this.
-   doc-sync does not handle workspaces correctly. It currently gets crate name manually and assumes `target` to be in the current directory, but it should be using the output of `cargo metadata` to
    determine these values.
-   Rustdoc code blocks such as ` ```ignore ` will appear as plain text after being converted to markdown. This may be fixable in the future when we stop using rustdoc as we will need a way to store
    data on items, but for now only normal rust code blocks (` ``` `) will appear as rust after being converted to markdown.
-   doc-sync does not play well with `#[doc = ...]`, especially include_str!. When we stop depending on rustdoc this should be fixed

Here is an incomplete list of situations doc-sync probably won't support ever:

-   Items expanded from macros. We would have to expand the macro to find the item it expanded to and then use rust-analyzer/rustc to undo the macro expansion to find the source of the doc comment
    (this may not even be possible, I haven't looked into it).

If there's something you are having issues with that isn't on this list, make a GitHub Issue and I'll add it to the list, or add support for it!

## How it works

### Doc comments -> Markdown (`to-markdown`)

rustdoc is run using a nightly toolchain so we can use the unstable JSON output feature. The JSON output is read and deserialized. doc-sync iterates through all items recognized by doc-sync and
creates markdown files for them.

### Markdown -> Doc comments (`from-markdown`)

The previously generated rustdoc JSON output is read and deserialized. doc-sync iterates through all the markdown files and extracts the rustdoc ID from each file. It uses this to get the rustdoc item
and related info from the JSON output. IF the docs have been changed, it continues.

At this point, it needs to update the docs in the file. If the item corresponds to the file itself, this is easy enough; simply use the doc comment parser to find the existing doc comment in the file
attributes, if there is a doc comment (otherwise it just inserts it at the top).

If it doesn't correspond to the item, it starts a process of looping through all parts of the path of the item relative to the file. For each part, it uses the rustdoc JSON output to get the item
kind. Using this information and syn, it resolves the item in the file. Then, it can get the source text of the item using proc_macro2's Span API. It then uses the doc comment parser to find and
replace an existing doc comment in the item (or insert a new one).

## Todo

-   Fix all the clippy lints (some seem to be caused by wrap-match, for example `clippy::useless_conversion`)
-   Remove dependency on rustdoc
    -   We should be using syn instead, it will give us more helpful information when converting back to doc comments
    -   Translating between rustdoc and syn types is messy
    -   rustdoc also doesn't support/include everything we could be supporting
-   Library so it can be run from xtasks and build.rs
-   Tests
-   Config file to avoid always having to supply arguments
