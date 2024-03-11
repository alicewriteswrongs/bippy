# Bippy

Bippy is a really simple and kinda bad http server. Good for locally serving a
directory of files when debugging something or when anything more complicated
is too much bother.

It's really really not for production use, I wrote it to learn a bit about
sockets and http, and while I don't know if I actually learned anything it does
somewhat work.

## Usage

To serve the contents of the current working directory just do:

```sh
bippy
```

To serve a particular directory you can do

```sh
bippy --dir docs/
```

Enjoy!
