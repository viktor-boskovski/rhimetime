# rhimetime

A primetime cli tool to find rhyming words by using a [scoring algorithm](https://axon.cs.byu.edu/Dan/673/papers/bay.pdf)
with an [IPA wordlist](https://github.com/open-dict-data/ipa-dict).


## usage 

```sh
$ rhimetime download list    # lists languages that you can download
$ rhimetime download de      # downloads the language wordlist
$ rhimetime download de,en   # downloads the languages, comma separated
$ rhimetime find text        # looks for words rhyming with text
$ rhimetime find --pure text # looks for pure rhymes only
```

## TODO

[ ] everything
  [ ] downloading system
  [ ] matching
  [ ] scoring

[ ] everything else 
  [ ] maybe TUI








