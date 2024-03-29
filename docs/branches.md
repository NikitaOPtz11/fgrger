# Branches


## Introduction

Branches are named pointers to revisions (just like they are in Git). You can
move them without affecting the target revision's identity. Branches
automatically move when revisions are rewritten (e.g. by `jj rebase`). You can
pass a branch's name to commands that want a revision as argument. For example,
`jj co main` will check out the revision pointed to by the "main" branch. Use
`jj branch list` to list branches and `jj branch` to create, move, or delete
branches. There is currently no concept of an active/current/checked-out branch.

## Remotes

Jujutsu records the last seen position on each remote (just like Git's
remote-tracking branches). You can refer to these with
`<branch name>@<remote name>`, such as `jj new main@origin`.

A remote branch can be associated with a local branch of the same name. It's
sometimes called a tracking branch. When you pull from a remote, any changes
compared to the current record of the remote's state will be propagated to the
tracking local branch. Let's say you run `jj git fetch --remote origin` and the
remote's "main" branch has moved so its target is now ahead of the local record
in `main@origin`. That will update `main@origin` to the new target. It will also
apply the change to the local branch `main`. If the local target had also moved
compared to `main@origin` (probably because you had run `jj branch set main`),
then the two updates will be merged. If one is ahead of the other, then that
target will be the new target. Otherwise, the local branch will be conflicted
(see the next "Conflicts" section for details).

Most commands don't show the tracking remote branch if it has the same target as
the local branch. The local branch (without `@<remote name>`) is considered the
branch's desired target. Consequently, if you want to update a branch on a
remote, you first update the branch locally and then push the update to the
remote. If a local branch also exists on some remote but points to a different
target there, `jj log` will show the branch name with an asterisk suffix
(e.g. `main*`). That is meant to remind you that you may want to push the branch
to some remote.

By default, the default remote branch (e.g. `main@origin`) will be tracked
automatically. You can use `jj branch track` to track existing remote branches
individually, or set `git.auto-local-branch = true` configuration to track all
new remote branches automatically.

### What does `git.auto-local-branch = true` actually do?

Jujutsu's fetch operation consist of several steps. First `jj git fetch` fetches
all Git refs under `refs/remotes/<remote name>`. Then Jujutsu stores these refs
as remote tracking branches. Finally, if `git.auto-local-branch = true`, Jujutsu
creates local branches for them. This is similar to Mercurial, which fetches all
its bookmarks (equivalent to Git branches) by default.

You can use `jj branch untrack <branch name>@<remote name>` to stop tracking
specific branches when fetching from specific remotes.

### Tracking a branch

To track a branch permanently use `jj branch track <branch name>@<remote name>`. 
It will now be imported as a local branch until you untrack it or it is deleted
on the remote. 

Example:

```sh
$ # List all available branches, as we want our colleague's branch.
$ jj branch list --all
$ # Find the branch.
$ # [...]
$ # Actually track the branch.
$ jj branch track <branch name>@<remote name> # Example: jj branch track my-feature@origin
$ # From this point on, branch <name> is tracked and will always be imported.
$ jj git fetch # Update the repository
$ jj new <name> # Do some local testing, etc.
```

### Untracking a branch

To no longer have a branch available in a repository, you can 
`jj branch untrack` it. After that subsequent fetches will no longer copy the 
branch into the local repository. 

Example: 

```sh
$ # List all local and remote branches.
$ jj branch list --all
$ # Find the branch we no longer want to track.
$ # [...]
# # Actually untrack it.
$ jj branch untrack <branch name>@<remote name> # Example: jj branch untrack stuff@origin
$ # From this point on, it won't be imported anymore. 
```

If you want to know the internals of branch tracking, consult the 
[Design Doc][design].


## Conflicts

Branches can end up in a conflicted state. When that happens, `jj status` will
include information about the conflicted branches (and instructions for how to
mitigate it). `jj branch list` will have details. `jj log` will show the branch
name with a question mark suffix (e.g. `main?`) on each of the conflicted
branch's potential target revisions. Using the branch name to look up a revision
will resolve to all potential targets. That means that `jj co main` will error
out, complaining that the revset resolved to multiple revisions.

Both local branches (e.g. `main`) and the remote branch (e.g. `main@origin`) can
have conflicts. Both can end up in that state if concurrent operations were run
in the repo. The local branch more typically becomes conflicted because it was
updated both locally and on a remote.

To resolve a conflicted state in a local branch (e.g. `main`), you can move the
branch to the desired target with `jj branch`. You may want to first either
merge the conflicted targets with `jj merge`, or you may want to rebase one side
on top of the other with `jj rebase`.

To resolve a conflicted state in a remote branch (e.g. `main@origin`), simply
pull from the remote (e.g. `jj git fetch`). The conflict resolution will also
propagate to the local branch (which was presumably also conflicted).

[design]: design/tracking-branches.md
