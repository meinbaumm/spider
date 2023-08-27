# Spider

Spider is a tool for web search on your favorite sites.

The idea is pretty simple: quick web search.

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
Usage: spider <COMMAND>

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
spider web github meinbaumm/spider
```

Find meaning of word healthy

```bash
spider web cambridge healthy
```

Google about Rust

```bash
spider web google "rust language"
```

If you want to open main page of website instead of long url for search, just don't write search term. This will open the main page instead of `https://www.urbandictionary.com/define.php?term=`

```bash
spider web urban
```

## How to define in which file to store web search urls

Spider will look for the `web-search-urls.spider` file in the current directory where the binary is located.

Also you may set the environment variable `SPIDER_FILE`. Just add this line to your .profile file:

```bash
export SPIDER_FILE="/home/username/web-search-urls.spider"
```
