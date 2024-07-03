build:
    cargo build

wordlist:
    make wordlists

compile:
    make compile

clean:
    make clean

test:
    cargo nextest run

test-linux:
    cross test --target armv7-unknown-linux-gnueabi