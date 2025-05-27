# OPSEC-0: Use, Organization, and Maintenance of the OAC Repository
- **Authors:** [Liam Monninger](mailto:liam@ramate.io)
- **Desiderata:** [ODE-0](../../../ode/oera-000-000-000-dulan/ode-000-000-000/README.md)
- **Contents:**
  - **[Summary](#summary)**
  - **[Motivation](#motivation)**
  - **[Specification](#specification)**
  - **[Correctness](#correctness)**
  - **[Agreeing](#agreeing)**
  - **[Dissenting](#dissenting)**
  - **[Appendix](#appendix)**

## Summary
We propose a soft specification of the use, organization, and maintenance of the OAC repository. We specify the usage of `-0` [Artifacts](../../../oglo/oera-000-000-000-dulan/oglo-000-000-000-artifact/README.md) as templates and likewise the organization at the `-0` stage as correct.

We suggest further clarifications of the use, organization, and maintenance of the OAC repository should be drafted and accepted as necessary.

We believe our specification adequately addresses [ODE-0.D1](../../../ode/oera-000-000-000-dulan/ode-000-000-000/README.md#d1-provide-templates-for-artifacts-and-describe-their-usage) and [ODE-0.D2](../../../ode/oera-000-000-000-dulan/ode-000-000-000/README.md#d1-provide-templates-for-artifacts-and-describe-their-usage).

## Motivation
[ODE-0](../../../ode/oera-000-000-000-dulan/ode-000-000-000/README.md) provides a clear motivation for the specification of the use, organization, and maintenance of the OAC repository: contribution. We provide our proposal to swiftly generate such a specification without overspecification and committing unnecessarily to structures which are not yet practiced.

## Specification
- **Contents:**
  - **[S1](#s1-use-of--0-artifacts-as-templates):** Use of `-0` [Artifacts](../../../oglo/oera-000-000-000-dulan/)
  - **[S2](#s2-shared-number-for-artifacts-via-github-shared-counter):** Shared number for [Artifacts](../../../oglo/oera-000-000-000-dulan/oglo-000-000-000-artifact/README.md) via [GitHub shared counter](https://github.com/orgs/community/discussions/69759)
  - **[S3](#s3-follow-directory-organization-of--0-stage):** Follow directory organization of `-0` stage

### S1: Use of `-0` [Artifacts](../../../oglo/oera-000-000-000-dulan/oglo-000-000-000-artifact/README.md) as templates

Each [Artifact](../../../oglo/oera-000-000-000-dulan/oglo-000-000-000-artifact/README.md) type has a `-0` member. This member MUST be used as the template document to the best of the contributor's ability when drafting any corresponding [Artifact](../../../oglo/oera-000-000-000-dulan/oglo-000-000-000-artifact/README.md) of the [Artifact](../../../oglo/oera-000-000-000-dulan/oglo-000-000-000-artifact/README.md) type.

We assert this partially addresses [ODE-0.D1](../../../ode/oera-000-000-000-dulan/ode-000-000-000/README.md#d1-provide-templates-for-artifacts-and-describe-their-usage), by providing the templates.

### S2: Shared number for [Artifacts](../../../oglo/oera-000-000-000-dulan/oglo-000-000-000-artifact/README.md) via [GitHub shared counter](https://github.com/orgs/community/discussions/69759)

An [Artifact](../../../oglo/oera-000-000-000-dulan/oglo-000-000-000-artifact/README.md) SHALL be assigned a number corresponding to the [Pull Request](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/about-pull-requests) in which it was first drafted. This number SHALL be used to title and reference the [Artifact](../../../oglo/oera-000-000-000-dulan/oglo-000-000-000-artifact/README.md). When named for a filesystem, the number SHALL be given padding to ensure lexicographic order.

We assert this partially addresses [ODE-0.D1](../../../ode/oera-000-000-000-dulan/ode-000-000-000/README.md#d1-provide-templates-for-artifacts-and-describe-their-usage), by asserting how templates should be filled in terms of numbering.

We assert this partially addresses [ODE-0.D2](../../../ode/oera-000-000-000-dulan/ode-000-000-000/README.md#d1-provide-templates-for-artifacts-and-describe-their-usage), by describing the organization of [Artifact](../../../oglo/oera-000-000-000-dulan/oglo-000-000-000-artifact/README.md) in terms of numbering.

### S3: Follow directory organization of `-0` stage

Contributors SHALL continue to place [Artifacts](../../../oglo/oera-000-000-000-dulan/oglo-000-000-000-artifact/README.md) into directories in a manner which matches the `-0` phase.

We assert this partially addresses [ODE-0.D2](../../../ode/oera-000-000-000-dulan/ode-000-000-000/README.md#d1-provide-templates-for-artifacts-and-describe-their-usage), by describing the organization of [Artifact](../../../oglo/oera-000-000-000-dulan/oglo-000-000-000-artifact/README.md) in terms directories within the repository.

## Correctness
We assert this is an arbitrary and adaptive standard that contains no obvious contradictions. We admit improvements to this standard when contradictions arise.

## Agreeing
- **[AGR-1: Liam Monninger](./agreeing/agr-001-liam-monninger/README.md):** argues that guide describes the exploratory nature of this initial phase well ([Liam Monninger](mailto:liam@ramate.io)).

## Dissenting
- **[DIS-1: Liam Monninger](./dissenting/dis-001-liam-monninger/README.md):** argues that the guide does not make it clear how to participate ([Liam Monninger](mailto:liam@ramate.io)).

## Appendix
$\emptyset$

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
