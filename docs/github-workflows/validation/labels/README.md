# Labels
The labels workflows are defined as follows:

- **[`labels.yml`](../../../../.github/workflows/labels.yml)**: describes the trigger

## Usage
The labels workflows will update the labels for this repository to correspond to those found in the `LABELS_JSON` hardcoded into [`labels.yml`](../../../../.github/workflows/labels.yml) whenever a commit is pushed to the GitHub remote.

The labels workflow will delete additional labels when run on `main` or when the commit message contains `debug(ci:labels:deletion)`.

> [!WARN]
> There are NOT currently any protections against this updating of labels. As a contributor, you should be mindful particularly not to remove labels without first checking with other contributors.

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
