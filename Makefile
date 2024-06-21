ENGLISH_SRC := data/ispell_wordlist/english.[0-3]
BRITISH_SRC := $(ENGLISH_SRC) data/ispell_wordlist/british.[0-2]
AMERICAN_SRC := $(ENGLISH_SRC) data/ispell_wordlist/american.[0-2]

WORD_FILES := data/english.words data/british.words data/american.words
CHARKOV_FILES := data/english.fictionary data/british.fictionary data/american.fictionary

all compile: $(CHARKOV_FILES)
wordlists: $(WORD_FILES)

data/english.words: $(ENGLISH_SRC)
	rg -IN '^[a-z]+$$' $^ | sort | sort -u > $@

data/american.words: $(AMERICAN_SRC)
	rg -IN '^[a-z]+$$' $^ | sort | sort -u > $@

data/british.words: $(BRITISH_SRC)
	rg -IN '^[a-z]+$$' $^ | sort | sort -u > $@

%.fictionary: %.words
	cargo run -- compile $< $@

clean:
	rm -f $(WORD_FILES) $(CHARKOV_FILES)

.PHONY: all clean wordlists