htaccess2json [![Build Status](https://travis-ci.org/brycefisher/htaccess2json.rs.svg)](https://travis-ci.org/brycefisher/htaccess2json.rs)
=============

Parses simple mod_rewrite directives like this:

```
\# Old.com
RewriteRule /old.html http://example.com/new [R=301,L]
RewriteRule /old/*.html http://example.com/new [R=301,L,QSA]
RewriteRule .* http://new.com [R=302,L]
```

It produces a json file like this:

```javascript
[{
  "pattern":"/old.html", "dest":"http://example.com/new", "flags":["R=301","L"],"domain":"old.com"
},{
  "pattern":"/old/*.html", "dest":"http://example.com/new", "flags":["R=301","L","QSA"]. "domain":"old.com"
},{
  "pattern":".*", "dest":"http://new.com", "flags":["R=302","L"], "domain":"old.com"
}]
```

## Building

```bash
$ curl https://static.rust-lang.org/rustup.sh | sudo sh
$ git clone https://github.com/brycefisher/htaccess2json.rs.git && htaccess2json.rs
$ cargo build
$ ./target/htaccess2json
```

The first command installs the [Rust compiler](http://www.rust-lang.org) and [package manager](http://crates.io), and it often requires several minutes to download the binaries depending on your connection speed.

Once you've built htaccess2json, you'll probably want to install it globally so that you can invoke it anywhere:

```bash
$ sudo ln -s `pwd`/target/htaccess2json /usr/bin/htaccess2json
```

## Command Line Usage

```bash
$ htaccess2json -i input -o output -d domain
Saved to disk. 178 rules captured, 2 rules skipped.
```

 * **-i** the input file, usually `.htaccess`. Since I often need to edit .htaccess, I recommend copying your htaccess so that you safely modify it.
 * **-o** output file, usually ending in `.json`
 * **-d** domain name for the site using the htaccess rule. 

All options are required.

## Limitations

 * Only RewriteRule directives are parsed
 * No support for RewriteCond at the moment (these are simply skipped)
 * No support for replacements in the destination (Ex: $1)
 * Must be a domain at the command line

