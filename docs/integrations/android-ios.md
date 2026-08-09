# Android and iOS integration

Consider a product with a Jetpack Compose Android app and a SwiftUI iOS app.
Both implement account signup, but they use different source trees, controls,
test frameworks, and build systems. EQM gives the product behavior one shared
semantic identity and keeps implementation/evidence target-specific.

## Recommended repository shape

```text
apps/android/                       Gradle and Compose application
apps/ios/                           Xcode and SwiftUI application
eqm.toml                            declares both target roots
eqm.lock                            exact adapter pins
eqm/contracts/auth.signup.toml      shared journey/surfaces/requirements
eqm/bindings/android.auth_signup.toml
eqm/bindings/ios.auth_signup.toml
eqm/policies/consumer.critical_flow.toml
eqm/profiles/audience.default.toml
eqm/runners/runner.android.toml
eqm/runners/runner.ios.toml
```

The complete form is in `examples/android-ios/`.

## Shared intent

Model behavior at the product level. For example, one requirement might state
that email is the initially selected signup identifier and another that a
valid six-digit OTP advances the journey. Give these requirements stable IDs
that do not contain Compose, SwiftUI, Gradle, Xcode, filenames, or test names.

Both target bindings reference those same requirement identities. The Android
binding maps them to Compose artifacts and Android test selectors; the iOS
binding maps them to SwiftUI artifacts and XCTest selectors. Each target can
change its internal architecture without forking product intent.

## Evidence mapping

Use one runner per execution environment. An Android runner can invoke a
repository-owned Gradle wrapper with a typed task and selector. An iOS runner
can invoke `xcodebuild` with explicit workspace, scheme, destination, and test
selector arguments. Declare programs and arguments directly; do not wrap them
in `sh -c` or interpolate a command string.

Policies can require behavioral evidence once per target and an end-to-end
obligation across both targets. A passing Android result never substitutes for
missing iOS evidence, and two target passes do not satisfy a separately
declared cross-target journey obligation.

## Adoption sequence

1. Copy `examples/android-ios/` and replace the two target roots and owners.
2. Reduce the sample contracts to one real signup slice.
3. Update artifact paths and evidence selectors in each binding.
4. Replace illustrative runner programs/arguments with bounded native test
   invocations already owned by the application repository.
5. Run `eqm validate`, then inspect `eqm obligations` and `eqm matrix
   conformance`.
6. Run `eqm verify --dry-run` for one target and selector.
7. Authorize execution separately, import/produce evidence, and run `eqm check`.
8. Add exact runtime facts and release records only after development checks
   are stable.

## CI and release use

EQM does not require repository-hosted workflow files. Any CI system can install
the pinned binary and invoke the CLI. A pull-request lane typically validates,
computes affected obligations from an exact commit, runs selected native tests,
and checks conformance. A release lane supplies an explicit profile and release
record and preserves pass, fail, and unknown as separate outcomes.

Do not claim product equivalence from contract symmetry alone. Equivalence is
earned from current, correctly scoped, sufficiently trusted evidence for both
targets and any required end-to-end scope.
