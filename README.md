# Httpcat

Inspired by the way Netcat makes it easy to shuffle data between computers over
a network, Httpcat makes it easy to shuffle data between your computer and a web
browser. While Netcat is a general purpose networking tool, Httpcat is really
just designed for sharing data between a web browser and stdout/stdin.

## Why?

Because I always have a few ports forwarded to my desktop, one of my favorite
ways to share files larger than something like Discord accepts is to use the
Python http server module. It's a super convenient way to share the contents of
a directory that's both non-technical for the other person and doesn't require a
bunch of accounts or other tools.

While that's a great way for me to share files with other people, it doesn't
help if I want the other person to share a file with me. It's also less great if
all I want to do is share a single file once instead of listing an entire
directory. If I were sending files between my own devices or other Linux users,
I might use Netcat. Although that's kind of fun, sometimes you just want
something that'll work without a lot of explanation.

Httpcat combines the flexibility of Netcat with how easy it is for someone else
to interact with the python directory server. Locally Httpcat interacts with
your stdin and stdout. You can use redirection to work with individual files,
you can type in or read your own messages directly from the terminal, or you can
pipe to or from other commands. For example, you can pipe the output of the tar
or zip commands to share collections of files. Similarly, you can pipe directly
into another app if you want to avoid intermediate files. This gives you a lot
of flexibility!

On the other side, Httpcat exposes a simple interface that anyone with a web
browser can use. They just upload their file like on any other website, press
submit, and that's it. It's similarly easy to download a file. The interface is
so simple it should load perfectly well even from a TUI web browser.

Ultimately that's the power of Httpcat: It gives technical users plenty of
flexibility by integrating with the rest of their environment without requiring
any specialized knowledge or skills from other users.

## How does it work?

The program starts a short lived http server. When it receives a GET request, it
replies back with a very basic HTML form. When the user submits the form, the
server either accepts the uploaded data or begins the download depending on how
the program was started. After receiving a form submission, the program exits.

## How can I

### Share one file with a friend?

`httpcat 8787 send map.png < ~/myMap.png`

### Share multiple files with a friend?

`zip - Cargo.lock Cargo.toml | httpcat 8787 send Cargo.zip`

### Receive a file from a friend

`httpcat 8787 receive > map.png`

### Receive multiple files with a friend?

`unzip =(httpcat 8787 receive)`

Zip files are a little strange, but if you're using zsh then the above should
work. Otherwise you'll need to write the zip file someplace temporary in
between.

### Send data not in plain text

Httpcat doesn't have any special support for https connections. Consider instead
setting up a reverse proxy that'll forward plain traffic to your port of choice.
You can use that port for other reasons if you need in the future.

### Prevent others from reading the data?

First, you probably need to setup https as mentioned above. There's no
authentication builtin (that might be a good feature to add later), but the http
server will only ever accept or share the data once before exiting. If you're
sharing a file and the other person receives it, you can be certain that no one
else downloaded it as well. Consider the amount of risk you're comfortable with
before using. The risk should be low, but there are certainly more secure ways
to share data.
