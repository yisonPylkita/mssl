# mssl - My Simple Scripting Language

[![Rust](https://github.com/yisonPylkita/mssl/actions/workflows/rust.yml/badge.svg)](https://github.com/yisonPylkita/mssl/actions/workflows/rust.yml)

Currently very much in progress!!! - only lexer kind of works right now

I wanted to understand how compiler works so I created one

### Code example

```perl
# My simple scripting language

# This is a comment
# Every comment starts with # and ends with \n

# This is a scripting language so we do not have a main function

# But we do have a normal functions
# They are declared this way
fn dummy_function(a: i32, b: i32) -> i32 {
    return a + b;
}

# We can have variable declarations
# Right now i32 is the only supported type
let first_num = 2;
let second_num = 5;
```

## Usage

### Just lexer result
```bash
cargo run -- lex "let x = 10; println(\"Hello world\");"
```

### Run provided code
```bash
cargo run -- run "let x = 10; println(\"Hello world\");"
```

### Vim config

I wrote configuration for Vim to support **mssl**. To enable it run
```sh
$ mkdir -p ~/.vim/syntax
$ cp misc/mssl.vim ~/.vim/syntax
```

and add this line to your `.vimrc`

```vim
au BufRead,BufNewFile *.mssl setfiletype mssl
```

