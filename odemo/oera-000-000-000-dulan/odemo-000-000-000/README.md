# ODEMO-0: Demonstrating the OAC Repository
- **Authors:** [Liam Monninger](mailto:liam@ramate.io)
- **Recording:** [ODEMO-0: Demonstrating the OAC Repository](https://www.loom.com/share/2f7b62b9c3c849289155c3bab5d76f96?sid=ba1c2fc6-2a30-46dc-91d4-a8db44485e05)

## Summary
This demo covers the usage of the OAC repository by contributors.

## Demo

### Artifacts
The primary product of this repository is its [Artifacts](/oglo/oera-000-000-000-dulan/oglo-000-000-000-artifact/). Familiarize yourself with them and ensure you understand what should be contributed under each.

### Events, Release Candidates, and Issues
Repository task planning follows a hierarchy. Events are planned activities with repositories targeting a particular date. Release Candidates are versions of the repository matching said features, thus they map to events. Issues describe bugs and features, thus more granularly describing Release Candidates.

> [!TIP]
> You can find high priority Events, Release Candidates, and Issues by using the table in the [Contributing](https://github.com/ramate-io/oac?tab=readme-ov-file#contributing) or [Contributing](https://github.com/ramate-io/oac/blob/main/CONTRIBUTING.md) file in the repository.

### Working with the Repository

#### Nix Flake
When working with the repository, ensure you have opened the [Nix Flake](https://nixos.wiki/wiki/Flakes) with `nix develop`. This will help provide a standardized environment and ensure that pre-commit hooks which benefit formatting are available.

> [!IMPORTANT]
> We **highly** recommend using the [Determinate Nix Installer](https://determinate.systems/blog/determinate-nix-installer/) as this helps further standardize the nix installation and is what is used in CI.

#### Pre-commit hooks
All of the scripts in [`.githooks/lib-pre-commit`](/.githooks/lib-pre-commit) should be run in a properly configured environment during the pre-commit phase of a Git workflow. These are at the time of writing:

- [`.githooks/lib-pre-commit/footers`](/.githooks/lib-pre-commit/footers) ensures the proper formatting of footers.
- [`.githooks/lib-pre-commit/index`](/.githooks/lib-pre-commit/index) automates the creation of era indices.
- [`.githooks/lib-pre-commit/links`](/.githooks/lib-pre-commit/links) checks for broken links and fragments.
- [`.githooks/lib-pre-commit/spellcheck`](/.githooks/lib-pre-commit/spellcheck) checks for spelling mistakes.

> [!TIP]
> Treat this as your linter and compiler for the OAC project.

#### Labels workflow
The [`labels.yml`](/.github/workflows/labels.yml) workflow manages GitHub labels creating and updating those which are need and deleting those which are not.

### Projects
We use GitHub Projects to groups issues across repositories and track projects. We do not make use of the more advanced features as we believe estimation and other forms of tracking should be performed primarily by understanding associated issues without too much summary.

#### Events and Projects
Events will have their own projects. For example, the [Hello World and Week 0 Readiness](https://github.com/orgs/ramate-io/projects/13) Project was generated for the [Hello World and Week 0 Readiness](https://github.com/ramate-io/oac/issues/17) Event, under which the generation of this **ODEMO** was an issue.

<!--OAC FOOTER: DO NOT REMOVE THIS LINE-->
---

<div align="center">
  <a href="https://github.com/ramate-io/oac">
    <picture>
      <source srcset="/assets/oac-inverted-transparent.png" media="(prefers-color-scheme: dark)">
      <img height="24" src="/assets/oac-transparent.png" alt="OAC"/>
    </picture>
  </a>
  <br/>
  <sub>
    <b>Ordered Atomic Collaboration (OAC)</b>
    <br/>
    &copy; 2025 <a href="https://github.com/ramate-io/oac">ramate-io/oac</a>
    <br/>
    <a href="https://github.com/ramate-io/oac/blob/main/LICENSE">MIT License</a>
    <br/>
    <a href="https://www.ramate.io">ramate.io</a>
  </sub>
</div>
