# diff_event_source
Git can operate like a database for the filesystem.

In preparation for a tool that can identify which files have changed in a repo so they can be incremntally published to a knowledge base. I created my first rust CLI that can give produce an event source from libgit2.

Ultimately, I wanted to make sure that if a file was moved it wouldn't be recreated to retain permalinks in the KB
