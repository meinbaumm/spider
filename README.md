# Spider

[![Crates info](https://img.shields.io/crates/v/spider-web-search.svg)](https://crates.io/crates/spider-web-search)

**Spider is a tool for web search on your favorite sites.**

The idea is pretty simple: quick web search.
It was born out of a desire to rewrite this zsh [web-search](https://github.com/ohmyzsh/ohmyzsh/blob/master/plugins/web-search/web-search.plugin.zsh) plugin.

For example, I often look up the meaning of English words on https://dictionary.cambridge.org/, but sometimes I am too lazy to go to my browser, go to the site and type in the word I want to search for. There can be many such search sites besides cambridge.org.

Thanks to spider, I can collect all the links in one file and quickly use it on terminal. For example you can create file with site names and urls, like this.

```bash
google          https://www.google.com/search?q=
duckduckgo      https://www.duckduckgo.com/?q=
github          https://github.com/search?q=
ecosia          https://www.ecosia.org/search?q=
goodreads       https://www.goodreads.com/search?q=
stackoverflow   https://stackoverflow.com/search?q=
wolframalpha    https://www.wolframalpha.com/input/?i=
archive         https://web.archive.org/web/*/
scholar         https://scholar.google.com/scholar?q=
multitran       https://www.multitran.com/
cambridge       https://dictionary.cambridge.org/dictionary/english/
urban           https://www.urbandictionary.com/define.php?term=
```

And then you can use spider.

```bash
Usage: spider-web-search <COMMAND>

Commands:
  web   Search on the web
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Examples

Open spider github repo

```bash
spider-web-search web github meinbaumm/spider
```

Find meaning of word healthy

```bash
spider-web-search web cambridge healthy
```

Google about Rust

```bash
spider-web-search web google "rust language"
```

If you want to open main page of website instead of long url for search, just don't write search term. This will open the main page instead of `https://www.urbandictionary.com/define.php?term=`

```bash
spider-web-search web urban
```

## How To

### How to define in which file to store web search urls

Spider will look for the `web-search-urls.spider` file in the current directory where the binary is located.

Also you may set the environment variable `SPIDER_FILE`. Just add this line to your .profile file:

```bash
export SPIDER_FILE="/home/username/web-search-urls.spider"
```

### How to install Spider

You can download suitable executable from https://github.com/meinbaumm/spider and copy it in some directory that is listed in your PATH (e.g. ~/bin).

You may also use cargo to install spider from crates.io:

`cargo install spider-web-search`

### How to build Spider

Spider is written in Rust. You may build it yourself with the help of cargo. Just clone this repository and execute the cargo build command in its main directory:

`cargo build --release`

### How to use spider instead of spider-web-search

Spider now has the name `spider-web-search` in crates.io.
But I prefer just `spider` more.
To avoid writing `spider-web-search` every time, you can add an alias to your .bashrc or .zshrc file.

If you installed spider with cargo:

```bash
which spider-web-search
```

Then copy the output and in your .bashrc or .zshrc file write

```bash
alias spider="~/./.cargo/bin/spider-web-search"
```

Next source file like. And use with pleasure :)

```bash
source .zshrc
```
