# Zinq

A comprehensive collection of packages designed to streamline rust development.

## Feature `error`

Instead of defining custom error types for each case, this module exports common error types
for reuse.

## Feature `path`

Tools/types for parsing a path string like `/hello/2/world` into segments for
use with other features, like `context`.

## Feature `context`

Contexts define a common api surface for sharing immutable context in rust.