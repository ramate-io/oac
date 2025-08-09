# ODEMO-0: Demonstrating the OAC Repository
- **Authors:** [Liam Monninger](mailto:liam@ramate.io)
- **Recording:** [ODEMO-0: Demonstrating the OAC Repository](https://www.loom.com/share/2f7b62b9c3c849289155c3bab5d76f96?sid=ba1c2fc6-2a30-46dc-91d4-a8db44485e05)
- **Transcript:** [Transcript](./Transcript.md)
- **Contents:**
  - **[Summary](#summary)**
  - **[Demo](#demo)**

## Summary
This demo covers the usage of the OAC repository by contributors.

## Demo
- **Contents:**
  - **[Artifacts](#artifacts)**
  - **[Events, Release Candidates, and Issues](#events-release-candidates-and-issues)**
  - **[Working with the repository](#working-with-the-repository)**
  - **[Projects](#projects)**

### Artifacts
- **Timestamped links:** [00:10](https://www.loom.com/share/2f7b62b9c3c849289155c3bab5d76f96?t=10&sid=a5ce05b0-047d-4efd-8d3e-e1a60308fe0c)

The primary product of this repository is its [Artifacts](/oglo/oera-000-000-000-dulan/oglo-000-000-000-artifact/). Familiarize yourself with them and ensure you understand what should be contributed under each.

#### `*-0` Artifacts
- **Timestamped links:** [14:04](https://www.loom.com/share/2f7b62b9c3c849289155c3bab5d76f96?t=844&sid=4a95ab6f-b8cf-4510-b6cb-56be54d014fb)

Use the `*-0` Artifacts, e.g. [OGUIDE-0](/oguide/oera-000-000-000-dulan/oguide-000-000-000/) as templates.

### Events, Release Candidates, and Issues
- **Timestamped links:** [01:10](https://www.loom.com/share/2f7b62b9c3c849289155c3bab5d76f96?t=70&sid=4a95ab6f-b8cf-4510-b6cb-56be54d014fb)

Repository task planning follows a hierarchy. Events are planned activities with repositories targeting a particular date. Release Candidates are versions of the repository matching said features, thus they map to events. Issues describe bugs and features, thus more granularly describing Release Candidates.

> [!TIP]
> You can find high priority Events, Release Candidates, and Issues by using the table in the [Contributing](https://github.com/ramate-io/oac?tab=readme-ov-file#contributing) or [Contributing](https://github.com/ramate-io/oac/blob/main/CONTRIBUTING.md) file in the repository.

### Working with the repository
- **Timestamped links:** [08:26](https://www.loom.com/share/2f7b62b9c3c849289155c3bab5d76f96?t=506&sid=4a95ab6f-b8cf-4510-b6cb-56be54d014fb)

When working with the repository, take care to standardize your environment and consider the common workflow.

#### Nix Flake
- **Timestamped links:** [08:26](https://www.loom.com/share/2f7b62b9c3c849289155c3bab5d76f96?t=506&sid=4a95ab6f-b8cf-4510-b6cb-56be54d014fb)

When working with the repository, ensure you have opened the [Nix Flake](https://nixos.wiki/wiki/Flakes) with `nix develop`. This will help provide a standardized environment and ensure that pre-commit hooks which benefit formatting are available.

> [!IMPORTANT]
> We **highly** recommend using the [Determinate Nix Installer](https://determinate.systems/blog/determinate-nix-installer/) as this helps further standardize the nix installation and is what is used in CI.

#### Pre-commit hooks
- **Timestamped links:** [10:00](https://www.loom.com/share/2f7b62b9c3c849289155c3bab5d76f96?t=600&sid=4a95ab6f-b8cf-4510-b6cb-56be54d014fb)

All of the scripts in [`.githooks/lib-pre-commit`](/.githooks/lib-pre-commit) should be run in a properly configured environment during the pre-commit phase of a Git workflow. These are at the time of writing:

- [`.githooks/lib-pre-commit/footers`](/.githooks/lib-pre-commit/footers) ensures the proper formatting of footers.
- [`.githooks/lib-pre-commit/index`](/.githooks/lib-pre-commit/index) automates the creation of era indices.
- [`.githooks/lib-pre-commit/links`](/.githooks/lib-pre-commit/links) checks for broken links and fragments.
- [`.githooks/lib-pre-commit/spellcheck`](/.githooks/lib-pre-commit/spellcheck) checks for spelling mistakes.

> [!TIP]
> Treat this as your linter and compiler for the OAC project.

#### Labels workflow
- **Timestamped links:** [12:45](https://www.loom.com/share/2f7b62b9c3c849289155c3bab5d76f96?t=765&sid=4a95ab6f-b8cf-4510-b6cb-56be54d014fb)

The [`labels.yml`](/.github/workflows/labels.yml) workflow manages GitHub labels creating and updating those which are need and deleting those which are not.

### Projects
- **Timestamped links:** [04:08](https://www.loom.com/share/2f7b62b9c3c849289155c3bab5d76f96?t=248&sid=4a95ab6f-b8cf-4510-b6cb-56be54d014fb), [06:38](https://www.loom.com/share/2f7b62b9c3c849289155c3bab5d76f96?t=398&sid=4a95ab6f-b8cf-4510-b6cb-56be54d014fb), [16:00](https://www.loom.com/share/2f7b62b9c3c849289155c3bab5d76f96?t=960&sid=4a95ab6f-b8cf-4510-b6cb-56be54d014fb)

We use GitHub Projects to groups issues across repositories and track projects. We do not make use of the more advanced features as we believe estimation and other forms of tracking should be performed primarily by understanding associated issues without too much summary.

#### Events and Projects
- **Timestamped links:** [06:38](https://www.loom.com/share/2f7b62b9c3c849289155c3bab5d76f96?t=398&sid=4a95ab6f-b8cf-4510-b6cb-56be54d014fb)

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
