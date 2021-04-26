# git-quickmsg
A (dumb) utility to generate commit messages


This little thing invokes `git status` and parses its output to pre-generate commit
messages. It is intended to be invoked via the `prepare-commitmsg` hook. Not providing
it with a file to write to will just result in it printing `\n\nQuick-committed`, which
in turn is intended to be used in a git/shell alias, like `git commit -v -m
"$(git-quickmsg)"`.
