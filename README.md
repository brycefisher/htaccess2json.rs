htaccess2json [![Build Status](https://travis-ci.org/brycefisher/htaccess2json.rs.svg)](https://travis-ci.org/brycefisher/htaccess2json.rs)
=============

Parses simple mod_rewrite directives like this:

```
# Old.com
RewriteRule /old.html http://example.com/new [R=301,L]
RewriteRule /old/*.html http://example.com/new [R=301,L,QSA]
RewriteRule .* http://new.com [R=302,L]
```

into json like this

```javascript
[{
  "pattern":"/old.html", "dest":"http://example.com/new", "flags":["R=301","L"], "domain":"old.com"
},{
  "pattern":"/old/*.html", "dest":"http://example.com/new", "flags":["R=301","L","QSA"], "domain":"old.com"
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

## Intended Use Case

If you have a large collection of redirect rules that you want to preserve (for SEO or whatever), but you want to move off Apache, it can take a lot of time to sift through the RewriteRules. Capturing a representation of this information as JSON should make it easy to port to a Node web app, or any other platform that can parse JSON easily.

I intend to incorporate this data from several domains/servers into a single "redirect server" which handles all the old domains.

## Limitations

 * Only RewriteRule directives are parsed
 * No support for RewriteCond at the moment (these are simply skipped)
 * No support for replacements in the destination (Ex: $1)
 * Must provide a domain at the command line
 * Supported flags are:
 ** R=301
 ** L
 ** R=302
 ** QSA

## Contributions Welcome! 

<a href="https://flattr.com/submit/auto?user_id=brycefisherfleig&url=https%3A%2F%2Fgithub.com%2Fbrycefisher%2Fhtaccess2json.rs" target="_blank"><img src="https://api.flattr.com/button/flattr-badge-large.png" alt="Flattr this" title="Flattr this" border="0"></a>

If you find this software useful, consider flattring. Pull requests and issues welcome as well.
