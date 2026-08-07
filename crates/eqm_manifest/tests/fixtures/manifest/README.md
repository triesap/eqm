# Manifest fixture corpus

`valid/minimal` is a complete repository-shaped workspace accepted by the real
loader. `negative` contains one focused invalid authority or replacement file
per boundary. The integration test copies the valid workspace and applies each
focused fixture so every failure travels through production discovery, parsing,
validation, conversion, and lock loading.

Filesystem-only cases (portable filename collisions, symlinks, and byte limits)
are materialized by the test because Git cannot represent them portably or
compactly.
