# git-quickmsg
A (dumb) utility to generate commit messages


This little thing invokes `git status` and parses its output to pre-generate commit
messages. It is intended to be invoked via the `prepare-commitmsg` hook, and expects to
get the path to the commit message file as its argument. Not providing it with a file
path to write to will instead result in it printing its message to standard out, which
in turn is intended to be used in a git/shell alias, like `git commit -v -m
"$(git-quickmsg)"`.
