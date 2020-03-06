# snapmail-dna

[![CircleCI](https://circleci.com/gh/h-be/snapmail-dna.svg?style=svg)](https://circleci.com/gh/h-be/snapmail-dna)

Holochain DNA for SnapMail from [Glass Bead Software](http://www.glassbead.com/), see [snapmail-happ](https://github.com/h-be/snapmail-happ) for main use app.


## Running

Before starting up the UI development, start up a Holochain Conductor with the SnapMail DNA. Here's how:

Enter a nix shell:

```
nix-shell --run snapmail-dna
```

This starts up the Conductor with a running instance of the DNA in it.

Leave this terminal open and running, as long as you're doing development.

## Building

To rebuild the DNA that holochain uses to run use the `hc` command:

```
nix-shell --run 'hc package'
```

Stop the running conductor (ctrl + c) and rerun the above again if you make changes to the DNA.

## Testing

To run the tests

```
nix-shell --run snapmail-test
```

## Releasing

Edit the `version.current` of the `config.nix` file, and set it to the desired version number of the release.

> TODO: notes about CHANGELOG.md and CHANGELOG-UNRELEASED.md

> TODO: notes about updating Release notes

Run

```
nix-shell --run hn-release-github
```

## Updating

To update the holonix version (and therefore the holochain binaries) edit the holonix property of `config.nix`.
