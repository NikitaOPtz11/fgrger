# Sparse Patterns v2 redeisgn

Authors: [Daniel Ploch](mailto:dploch@google.com)

**Summary:** This Document documents a redesign of the sparse patterns list
stored in the JJ working copy, in order to facilitate several desirable
improvements. It covers both the migration path and the planned end state.

## Current State

Sparse patterns are encoded as a list of prefix strings in the working copy
proto and as a list of RepoPaths in Rust code. Patterns can be added to or
removed from this list, and the list can be converted into a Matcher which
visits only paths matching at least one prefix in the list. Additionally,
the sparse set is ephemeral, tracked only in the working copy proto. We aim
to improve Sparse Patterns through several steps.

## Redesign Overview

The redesign (hereafter "Sparse Patterns v2") aims to accomplish three goals:

1.  Sparse Patterns should be versioned with the working copy
1.  Sparse Patterns should support more flexible matching rules
1.  Sparse Patterns should support client path remapping

The ability to support sub-exclusions and non-recursive inclusions is vital to
working with large mono repos with gigantic subtrees. Sometimes it is necessary
to include a relevant parent directory with code, but exclude giant
subdirectories with test data, images, i18n, etc.  Non-recursive inclusions are
vital for very wide directories.

With the added complexity and the potential for custom integrations to go wrong
for different pattern sets, versioning the Sparse Patterns set becomes very
useful as well.

### Versioning and Storage

The SparsePatterns [proto](https://github.com/martinvonz/jj/blob/066f31a15d6605c596928ebd8b01096120f66b7d/lib/src/protos/working_copy.proto#L35)
will first have its definition moved into the op_store for persistent storage.
Like all other objects, it will gain a content-based hash id for unique
reference. For performance, the default 'everything' SparsePattern (a list
containing one element, the empty string) will be special-cased to avoid storage
penalties. The OpStore trait will gain read+write methods for SparsePatterns
objects, that function as expected.

The [View](https://github.com/martinvonz/jj/blob/066f31a15d6605c596928ebd8b01096120f66b7d/lib/src/protos/op_store.proto#L61)
object will change to store structured data for each working copy:

```
# Before
map<string, bytes> wc_commits_ids = ...;

# After
message WorkingCopyInfo {
    bytes wc_commit_id = 1;
    bytes sparse_patterns_id = 2;
}
map<string, WorkingCopyInfo> wc_infos = ...;
```

The op store will be automatically migrated to the new form when the relevant
jj release is run, and support will be maintained for old-format Views for at
least 4 minor releases as is standard.

In this way, jj commands which modify the Sparse Patterns set will be
rearchitected to persist the new pattern to the current View first, and apply
the changes to the working copy second. The working copy proto will change
from storing the full list of sparse patterns to just the SparsePatternId, to
determine whether or not the current Sparse Patterns set tracked in the View is
stale or not. SparsePatterns can thus be modified with `--ignore-working-copy`
to support incremental edits without splatting the results into the working
copy (expensive!) on every operation.

### Type Evolution

Before all else, SparsePatterns should be moved into a proper struct with
encapsulation, rather than a bare list of RepoPaths. This will allow further
evolutions of the type without breaking legacy clients that only know how to
provide a list of prefixes or receive a Matcher.

Sparse Patterns in text form will graduate to a `<type>:<path>` syntax, where
`<path>` will be interpreted as `include:<path>` for backwards-compatibility on
the command-line. Any unfortunate souls with sparse patterns where the first
path component includes a colon will be safe (the migration will preserve the
paths), but will need to use explicit syntax going forward when running
`jj sparse` commands. Inside JJ, `<type>` will be a strongly typed enum with a
Proto equivalent.

The relevant types in Sparse Patterns v2 will be:

- `include:<path>`: Match a specific prefix/dir
- `exclude:<path>`: Exclude a specific prefix/dir
- `files:<path>`: Include only the files in `<path>`, not subdirectories

The `jj sparse` family of commands will be altered to support these specific
patterns for adding and removing. We may also consider adding special flags
like `jj sparse set --exclude <path>` as a short-hand for
`jj sparse set --add exclude:<path>`.

#### Exclusion Ordering

The ordering of include and exclude rules in a SparsePattern matters. For the
purpose of evaluation, any rule whose path is a suffix of another rule's path
is treated as evaluated _after_ the other rule. Thus, the set of rules:

```
include:foo
exclude:foo/bar
include:foo/bar/baz
exclude:foo/bar/baz/qux
```

Is interpreted to match "foo/file.txt", not-match "foo/bar/file.txt", match
"foo/bar/baz/file.txt", and non-match "foo/bar/baz/qux/file.txt". If a user
specifies both `include:foo` and `exclude:foo`, the exclusion will override the
inclusion for the purpose of determinism. Internally, this will likely be
represented as a tree of nested rules keyed by path component, but the
representation is immaterial to the definition.

### Mapping

For various reasons, it is desirable to be able to transparently rename a
directory foo -> bar in the working copy without actually renaming the directory
in the public repo. Sparse Patterns are the ideal place to implement such a
feature, since they are deeply tied to the working copy.

Sparse Patterns will gain a 'map' field which is functionally a list of
`(<type>, <src_path>, <dst_path>)` tuples. `<type>` will be an enum indicating
'recursive' or 'non-recursive', while `<src_path>` and `<dst_path>` will be
RepoPaths indicating the source directory being renamed, and its renamed
destination. As with inclusions and exclusions, the most specific match for a
rename will apply.

How command-line paths are handled when a client map is enabled is an open
question - some commands should clearly accept dst_paths, while others should
probably accept src_paths. Any path which is interpreted as a relpath in the
working copy should definitely be considered a dst_path; we will likely want to
define strong types for these two kinds of paths to ensure they are always
converted correctly.

#### Sparse + Mapping

The sparse set will always apply to src_paths, with the mapping laid on top.
Thus, if a `"foo" -> "bar"` mapping exists, the sparse set must contain "foo",
not "bar", for the directory to be materialized.
