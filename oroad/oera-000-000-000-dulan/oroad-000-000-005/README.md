# OROAD-5: Week 0
- **Authors:** [Liam Monninger](liam@ramate.io)
- **Contents:**
    - **[Summary](#summary)**
    - **[Roadmap](#roadmap)**
    - **[Agreeing](#agreeing)**
    - **[Dissenting](#dissenting)**
    - **[Appendix](#appendix)**

## Summary
**OROAD-5** is a roadmap for the first week of the OAC project--produced once necessary preparations were made to begin its development.

**OROAD-5** rearranges the priorities of [OROAD-0](/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md), outlines procedural bootstrapping, specifies initial progression against foundational papers, suggests several memos, and proposes development of utility APIs.

> [!TIP]
> **[[Liam Monninger]](liam@ramate.io)**
>
> The inclusion of memos and utility APIs may seem out of place or unnecessary. However, the intent of pursuing these in Week 0 is partially to ramp back up from a "garden leave" enforced during necessary preparations.
>
> Returning to smaller ideas which were first produced under a state of mental heat may help to restore the initial inertia of this project's development.

> [!IMPORTANT]
> **[[Liam Monninger]](liam@ramate.io)**
>
> Some of the contents of this roadmap refer to efforts which should in the future be placed outside of OAC. However, I am currently using the OAC repository to plan, while these other structures are being put in place.
>
> I further believe this is reasonable given--from one perspective--Ramate and Robles are at the service of pursuing OAC.

> [!NOTE]
> **[[Liam Monninger]](liam@ramate.io)**
>
> I may use the expression "necessary preparations" to refer to the period between May 2025 and July 2025. This is the period in which informal discussions of the OAC were advanced in order to arrange its orderly commencement.
>
> Unfortunately, the content of these discussions is generally exempt from the strong openness of this organization owing to the sensitivities of parties involved.
>
> At the immediate time of writing, I have not codified this term. However, I may soon do so.

> [!TIP]
> > **[[Liam Monninger]](liam@ramate.io)**
>
> OROAD-5 also establishes the practice of further specifying and making adjustments to a higher-order OROAD via a lower-order OROAD.

## Roadmap
> [!WARNING]
> Ensure **All leads** contains list of all leads from milestones below.
>
> **[AI Prompt]**
>
> Help contributors to ensure the above.

- **All leads:** [Liam Monninger](liam@ramate.io)
- **Contents:**
    - **[T1](#t1-organization-and-updating-oroad-0):** Organization and Updating OROAD-0
    - **[T2](#t2-memos):** Memos
    - **[T3](#t3-bfa-first-draft):** BFA First Draft
    - **[T4](#t4-emframed-api):** `emframed` API
    - **[T5](#t5-cite-api):** `cite` API
    - **[T6](#t6-roadline-api):** `roadline` API

### T1: Organization and Updating OROAD-0
> [!IMPORTANT]
> **T1** prioritizes organizing efforts across OAC, Ramate, and Robles.

- **Starts:** T1 + 0 days
- **Depends-on:** $\emptyset$
- **Ends:** T1 + 1 day
- **Contents:**
    - **[T1.1](#t11-redraft-oroad-0)**: Redraft [OROAD-0](/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md)
    - **[T1.2](#t12-clean-and-update-oac-repository)**: Clean and update [OAC](https://github.com/ramate-io/oac) repository
    - **[T1.3](#t13-clean-and-update-ramate-repository)**: Clean and update [Ramate](https://github.com/ramate-io/ramate) repository
    - **[T1.4](#t14-clean-and-update-robles-repository)**: Clean and update [Robles](https://github.com/ramate-io/robles) repository
    - **[T1.5](#t15-update-ramateio)**: Clean and update [ramate.io](https://ramate.io)
    - **[T1.6](#t16-clean-hardware-and-reset-developer-environment)**: Clean hardware and reset developer environment
    - **[T1.7](#t17-wardley-maps)**: Wardley Maps

**T1** is a one day scramble to update high-level roadmaps and organizational understanding, as well as to ensure that the key repositories [OAC](https://github.com/ramate-io/oac), [Ramate](https://github.com/ramate-io/ramate), and [Robles](https://github.com/ramate-io/robles) are prepared for usage for development.

**T1** seeks to accomplish the following itemized objectives:

#### T1.1: Redraft [OROAD-0](/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md)
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

[OROAD-0](/oroad/oera-000-000-000-dulan/oroad-000-000-000/README.md) should be updated to better reflect externalities and a re-evaluated progression of the project. This does not require a termination of the roadmap and replacement, as OROAD-0 was not aggressively pursued.

#### T1.2: Clean and update [OAC](https://github.com/ramate-io/oac) repository
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

The [OAC](https://github.com/ramate-io/oac) repository could use a tidy up.

#### T1.3: Clean and update [Ramate](https://github.com/ramate-io/ramate) repository
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

The [Ramate](https://github.com/ramate-io/ramate) repository is still mainly just a copy of the [OAC](https://github.com/ramate-io/oac) repository. It should be updated to reflect the unique specification, documentation, and planning performed under the [Ramate](https://github.com/ramate-io/ramate) organization.

#### T1.4: Clean and update [Robles](https://github.com/ramate-io/robles) repository
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

The [Robles](https://github.com/ramate-io/robles) repository is still mainly just a copy of the [OAC](https://github.com/ramate-io/oac) repository. It should be updated to reflect the unique specification, documentation, and planning performed under the [Robles](https://github.com/ramate-io/robles) project.

#### T1.5: Update [ramate.io](https://ramate.io)
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

The [ramate.io](https://ramate.io) website should be updated to reflect the now active status or Ramate, OAC, and Robles. It should link to the key repositories and, time permitting, provide in-site markdown rendering of their documentation.

> [!NOTE]
> **[[Liam Monninger]](mailto:liam@ramate.io)**
>
> It may also be worthwhile to attempt to migrate to a Rust-based web framework as wbe projects going forward will generally be developed in this manner.

#### T1.6: Clean hardware and reset developer environment
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

This is mainly a note to self to make sure to take the time to remove unnecessary files and ensure developer tools like Cursor are properly configured for best usage. Many subscriptions have expired and devices could use cleaning.

#### T1.7: Wardley Maps
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

In an effort to develop a strong high-level organizational understanding, we'll be attempting to apply the Wardley Mapping technique. Wardley Maps are traditionally applied to the business and perhaps more relevant to simply Ramate in the long-run. But, to feel out the technique, we'll attempt to Wardley Map Ramate, OAC, and Robles individually as well as jointly and document the process.

### T2: Memos
> [!IMPORTANT]
> **T2** focuses on recording memos which were ideated upon during the necessary preparations period.

- **Starts:** T1 + 1 day
- **Depends-on:** [T1](#t1-organization)
- **Ends:** T2 + 5 days
- **Contents:**
    - **[T2.1](#t21-share-and-gather-feedback-on-oart-1-bfa)**: Share and gather feedback on [OART-1: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-001-bfa/README.md)
    - **[T2.2](#t22-share-and-gather-feedback-on-oart-2-collaborative-transaction-routing)**: Share and gather feedback on [OART-2: Collaborative Transaction Routing](../../../oart/oera-000-000-000-dulan/oart-000-000-002-ctr/README.md)
    - **[T2.3](#t23-implement-and-document-proposal-standards)**: Implement and document proposal standards, contributor guidelines, and implementation governance
    - **[T2.4](#t24-complete-gwrdfa-reference-implementation)**: Complete [`gwrdfa`](https://github.com/ramate-io/gwrdfa) reference implementation
    - **[T2.5](#t25-complete-srcavei-reference-implementation)**: Complete [`srcavei`](https://github.com/ramate-io/srcavei) reference implementation
    - **[T2.6](#t26-continue-development-of-fuste)**: Continue development of [`fuste`](https://github.com/ramate-io/fuste) as a lower priority task

**T2** focuses on validating the initial content and establishing contribution frameworks for Ordered Atomic Collaboration (OAC) and [`robles`](https://github.com/ramate-io/robles).

**T2** seeks to accomplish the following itemized objectives:

#### T2.1: Write "Vectors philosophy of work"
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)s

#### T2.2: Write "Wardley Maps in the context of compute infrastructure"
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T2.3: Write "On the rarity of sub-sampling protocols"
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T2.4: Write "Traits and coroutines"
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T2.5: Write "Verification and traits"
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T2.6: Write "The value in decentralization"
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T2.7: Write "What could be a quantum perspective on distributed tasks?"
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T2.8: Write "Topos theory and distributed systems"
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T2.9: Write "Topos theory and classes of algorithms"
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

### T3: BFA First Draft
> [!IMPORTANT]
> **T3** focuses on rapidly producing a first draft of BFA an concretely identifying weakpoints.

- **Starts:** T1 + 1 day
- **Depends-on:** [T1](#t1-organization)
- **Ends:** T3 + 5 days
- **Contents:**
    - **[T3.1](#t31-continue-sharing-and-updating-oart-1-bfa)**: Continue sharing and updating [OART-1: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-001-bfa/README.md)
    - **[T3.2](#t32-continue-sharing-and-updating-oart-2-collaborative-transaction-routing)**: Continue sharing and updating [OART-2: Collaborative Transaction Routing](../../../oart/oera-000-000-000-dulan/oart-000-000-002-ctr/README.md)
    - **[T3.3](#t33-develop-fuste-mvp)**: Develop [`fuste`](https://github.com/ramate-io/fuste) MVP
    - **[T3.4](#t34-use-fuste-mvp-to-develop-centralized-embedded-database)**: Use [`fuste`](https://github.com/ramate-io/fuste) MVP to develop centralized embedded database

**T3** focuses on continued validation of core concepts and the development of the Fuste MVP as a proof of concept for Ordered Atomic Collaboration (OAC).

**T3** seeks to accomplish the following itemized objectives:

#### T3.1: Produce Latex shell draft of [OART-2: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-002-bfa/README.md)
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T3.2: Provide annotated bibliography for [OART-2: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-002-bfa/README.md)
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T3.3: Write OLOG identifying weakpoints in [OART-2: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-002-bfa/README.md) shell draft
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

### T4: `emframed` API
> [!IMPORTANT]
> **T4** pursues an initial implementation of the `emframed` API which forms the basis for reasoning about highly-constrained contexts used throughout the initial OAC implementation.

- **Starts:** T1 + 1 day
- **Depends-on:** [T1](#t1-organization)
- **Ends:** T4 + 5 days
- **Contents:**
    - **[T3.1](#t31-continue-sharing-and-updating-oart-1-bfa)**: Continue sharing and updating [OART-1: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-001-bfa/README.md)
    - **[T3.2](#t32-continue-sharing-and-updating-oart-2-collaborative-transaction-routing)**: Continue sharing and updating [OART-2: Collaborative Transaction Routing](../../../oart/oera-000-000-000-dulan/oart-000-000-002-ctr/README.md)
    - **[T3.3](#t33-develop-fuste-mvp)**: Develop [`fuste`](https://github.com/ramate-io/fuste) MVP
    - **[T3.4](#t34-use-fuste-mvp-to-develop-centralized-embedded-database)**: Use [`fuste`](https://github.com/ramate-io/fuste) MVP to develop centralized embedded database

**T3** focuses on continued validation of core concepts and the development of the Fuste MVP as a proof of concept for Ordered Atomic Collaboration (OAC).

**T3** seeks to accomplish the following itemized objectives:

#### T4.1: Produce initial `emframed` API
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T4.2: Provide embedded demo of `emframed` API
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T4.3: Provide guides for using `emframed` to indicate common patterns and reason through its qualities
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T4.4: Generate specification for `emframed` API
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

### T5: `cite` API
> [!IMPORTANT]
> **T5** pursues an initial implementation of the `cite` API which forms the basis for implementation invalidation and authority control to be used through Ramate, Robles, and OAC.

- **Starts:** T1 + 1 day
- **Depends-on:** [T1](#t1-organization)
- **Ends:** T5 + 5 days
- **Contents:**
    - **[T3.1](#t31-continue-sharing-and-updating-oart-1-bfa)**: Continue sharing and updating [OART-1: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-001-bfa/README.md)
    - **[T3.2](#t32-continue-sharing-and-updating-oart-2-collaborative-transaction-routing)**: Continue sharing and updating [OART-2: Collaborative Transaction Routing](../../../oart/oera-000-000-000-dulan/oart-000-000-002-ctr/README.md)
    - **[T3.3](#t33-develop-fuste-mvp)**: Develop [`fuste`](https://github.com/ramate-io/fuste) MVP
    - **[T3.4](#t34-use-fuste-mvp-to-develop-centralized-embedded-database)**: Use [`fuste`](https://github.com/ramate-io/fuste) MVP to develop centralized embedded database

**T3** focuses on continued validation of core concepts and the development of the Fuste MVP as a proof of concept for Ordered Atomic Collaboration (OAC).

**T3** seeks to accomplish the following itemized objectives:

#### T5.1: Produce initial `cite` API
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T5.2: Provide demo of `cite` API using some specification within Ramate, Robles, or OAC
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T5.4: Generate specification for `cite` API
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

### T6: `roadline` API
> [!IMPORTANT]
> **T5** pursues an initial implementation of the `roadline` API which forms the basis for visual renderings of roadmaps and tilines

- **Starts:** T1 + 1 day
- **Depends-on:** [T1](#t1-organization)
- **Ends:** T6 + 5 days
- **Contents:**
    - **[T3.1](#t31-continue-sharing-and-updating-oart-1-bfa)**: Continue sharing and updating [OART-1: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-001-bfa/README.md)
    - **[T3.2](#t32-continue-sharing-and-updating-oart-2-collaborative-transaction-routing)**: Continue sharing and updating [OART-2: Collaborative Transaction Routing](../../../oart/oera-000-000-000-dulan/oart-000-000-002-ctr/README.md)
    - **[T3.3](#t33-develop-fuste-mvp)**: Develop [`fuste`](https://github.com/ramate-io/fuste) MVP
    - **[T3.4](#t34-use-fuste-mvp-to-develop-centralized-embedded-database)**: Use [`fuste`](https://github.com/ramate-io/fuste) MVP to develop centralized embedded database

**T3** focuses on continued validation of core concepts and the development of the Fuste MVP as a proof of concept for Ordered Atomic Collaboration (OAC).

**T3** seeks to accomplish the following itemized objectives:

#### T5.1: Produce initial `roadline` API
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T5.2: Provide demo of `roadline` API, ideally using a roadmap from one of the Ramate, Robles, or OAC
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T5.3: Generate specification for `roadline` API
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

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
