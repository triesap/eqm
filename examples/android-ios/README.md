# Android and iOS equivalence

This example models one signup requirement across Android/Jetpack Compose and
iOS/SwiftUI targets. The `eqm/` directory is authored product authority;
`eqm.toml` selects it and declares both application roots; and `eqm.lock` pins
adapter inputs.

Copy this directory's contents into the root of a Git repository, then run
`eqm validate` to validate the model and `eqm check` to evaluate whether each
target has current trusted evidence. The runner files show how a product can
map EQM evidence selectors and result paths into Gradle and Xcode test commands.
Replace the illustrative paths, owners, and commands with the consuming
application's real values.
