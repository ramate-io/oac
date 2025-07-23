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
> **[[Liam Monninger]](liam@ramate.io)**
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
    - **[T2.1](#t21-write-vectors-philosophy-of-work)**: Write "Vectors philosophy of work"
    - **[T2.2](#t22-write-wardley-maps-in-the-context-of-compute-infrastructure)**: Write "Wardley Maps in the context of compute infrastructure"
    - **[T2.3](#t23-write-on-the-rarity-of-sub-sampling-protocols)**: Write "On the rarity of sub-sampling protocols"
    - **[T2.4](#t24-write-traits-and-coroutines)**: Write "Traits and coroutines"
    - **[T2.5](#t25-write-verification-and-traits)**: Write "Verification and traits"
    - **[T2.6](#t26-write-the-value-in-decentralization)**: Write "The value in decentralization"
    - **[T2.7](#t27-write-what-could-be-a-quantum-perspective-on-distributed-tasks)**: Write "What could be a quantum perspective on distributed tasks?"
    - **[T2.8](#t28-write-topos-theory-and-distributed-systems)**: Write "Topos theory and distributed systems"
    - **[T2.9](#t29-write-topos-theory-and-classes-of-algorithms)**: Write "Topos theory and classes of algorithms"

**T2** focuses on validating the initial content and establishing contribution frameworks for Ordered Atomic Collaboration (OAC) and [`robles`](https://github.com/ramate-io/robles).

**T2** seeks to accomplish the following itemized objectives:

#### T2.1: Write "Vectors philosophy of work"
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Write a memo describing the "vectors" philosophy of work which describes summing perspectives and efforts to meet a goal.

This is advanced to describe an initial sense of the philosophy of work behind the Ramate, OAC, and Robles.

This should most like be an RMEMO.

#### T2.2: Write "Wardley Maps in the context of compute infrastructure"
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Write a memo describing the particular application of Wardley Maps in the context of compute infrastructure and related musings.

This is advanced to develop a better sense of Wardley Maps in the contexts relevant to Ramate, OAC, and Robles, and to record learnings.

#### T2.3: Write "On the rarity of sub-sampling protocols"
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Write a memo describing the current state of protocols matching or similar to the sub-sampling techniques which are slated for formalization under BFA.

This is advanced to provide a brief motivation for BFA and prepare to contend with detractions.

#### T2.4: Write "Traits and coroutines"
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Write a memo describing the fundamental types of coroutines which may be described w.r.t. to memory ownership in Rust.

This will serve to motivate `emframed`.

#### T2.5: Write "Verification and traits"
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Write a memo describing approaches to formal verification in Rust w.r.t. traits. Emphasize the [Assume-Guarantee](https://ntrs.nasa.gov/api/citations/20060017073/downloads/20060017073.pdf) approach.

#### T2.6: Write "The value in decentralization"
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Write a memo philosophizing on the value in decentralization. Amongst other things, this shall help add credence to the value of consumer applications built on OAC and Robles.

#### T2.7: Write "What could be a quantum perspective on distributed tasks?"
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Write a memo inquiring as to the possibilities for leverage quantum compute in the distributed task setting.

#### T2.8: Write "Topos theory and distributed systems"
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Write a memo providing an initial review and attempted toy examples of using topos theory in distributed systems.

#### T2.9: Write "Topos theory and classes of algorithms"
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Write a memo provoking the notion of using topos theory to better describe classes of algorithms.

### T3: BFA First Draft
> [!IMPORTANT]
> **T3** focuses on rapidly producing a first draft of BFA an concretely identifying weakpoints.

- **Starts:** T1 + 1 day
- **Depends-on:** [T1](#t1-organization)
- **Ends:** T3 + 5 days
- **Contents:**
    - **[T3.1](#t31-produce-latex-shell-draft-of-oart-2-bfa)**: Produce Latex shell draft of [OART-2: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-002-bfa/README.md)
    - **[T3.2](#t32-provide-annotated-bibliography-for-oart-2-bfa)**: Provide annotated bibliography for [OART-2: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-002-bfa/README.md)
    - **[T3.3](#t33-write-olog-identifying-weakpoints-in-oart-2-bfa-shell-draft)**: Write OLOG identifying weakpoints in [OART-2: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-002-bfa/README.md) shell draft

**T3** focuses on producing a first draft of the BFA paper.

**T3** seeks to accomplish the following itemized objectives:

#### T3.1: Produce Latex shell draft of [OART-2: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-002-bfa/README.md)
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Simply write the Latex shell draft of [OART-2: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-002-bfa/README.md).

#### T3.2: Provide annotated bibliography for [OART-2: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-002-bfa/README.md)
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Write the annotated bibliography for [OART-2: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-002-bfa/README.md). This should allow for better and more complete comparative review in the near future.

#### T3.3: Write OLOG identifying weakpoints in [OART-2: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-002-bfa/README.md) shell draft
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Try to identify weakpoints in the initial BFA conceptualization. This should help inform planning which respects where the paper will need to be bolstered most.

### T4: `emframed` API
> [!IMPORTANT]
> **T4** pursues an initial implementation of the `emframed` API which forms the basis for reasoning about highly-constrained contexts used throughout the initial OAC implementation.

- **Starts:** T1 + 1 day
- **Depends-on:** [T1](#t1-organization)
- **Ends:** T4 + 5 days
- **Contents:**
    - **[T4.1](#t41-produce-initial-emframed-api)**: Produce initial `emframed` API
    - **[T4.2](#t42-provide-embedded-demo-of-emframed-api)**: Provide embedded demo of `emframed` API
    - **[T4.3](#t43-provide-guides-for-using-emframed-to-indicate-common-patterns-and-reason-through-its-qualities): Provide guides for using `emframed` to indicate common patterns and reason through its qualities
    - **[T4.4](#t44-generate-specification-for-emframed-api)**: Generate specification for `emframed` API

**T4** pursues an initial implementation of the `emframed` API which forms the basis for reasoning about highly-constrained contexts used throughout the initial OAC implementation.

**T4** seeks to accomplish the following itemized objectives:

#### T4.1: Produce initial `emframed` API
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

The main focus here will be on ergonomics and clarity of the base traits and producing binding logic for working with common runtimes.

#### T4.2: Provide embedded demo of `emframed` API
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Flash a device with a program written using the `emframed` API and record a video.

#### T4.3: Provide guides for using `emframed` to indicate common patterns and reason through its qualities
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Provide guides for using `emframed`. Identify common patterns and reason through its qualities. Not only will this potentially make it more understandable for future users, but it will also help to understand the strengths and weaknesses in the current API.

#### T4.4: Generate specification for `emframed` API
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Generate a specification, likely RASPEC, for `emframed`. We ask for to be done after the initial concept in this case to allow for a more informed specification to be generated.

### T5: `cite` API
> [!IMPORTANT]
> **T5** pursues an initial implementation of the `cite` API which forms the basis for implementation invalidation and authority control to be used through Ramate, Robles, and OAC.

- **Starts:** T1 + 1 day
- **Depends-on:** [T1](#t1-organization)
- **Ends:** T5 + 5 days
- **Contents:**
    - **[T5.1](#t51-produce-initial-cite-api)**: Produce initial `cite` API
    - **[T5.2](#t52-provide-demo-of-cite-api-using-some-specification-within-ramate-robles-or-oac)**: Provide demo of `cite` API using some specification within Ramate, Robles, or OAC
    - **[T5.3](#t53-generate-specification-for-cite-api)**: Generate specification for `cite` API

**T5** pursues an initial implementation of the `cite` API which forms the basis for implementation invalidation and authority control to be used through Ramate, Robles, and OAC.

**T3** seeks to accomplish the following itemized objectives:

#### T5.1: Produce initial `cite` API
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

The initial `cite` API shall contain macros for citing and reporting un-cited or discrepant implementations. It will implement this for several common use cases with appropriate parameterization.

#### T5.2: Provide demo of `cite` API using some specification within Ramate, Robles, or OAC
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Provide a demo of using `cite` with Ramate, Robles, or OAC to trigger warnings. Record a video.

#### T5.3: Generate specification for `cite` API
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Generate a specification, likely RASPEC, for `cite`. We ask for to be done after the initial concept in this case to allow for a more informed specification to be generated.

### T6: `roadline` API
> [!IMPORTANT]
> **T6** pursues an initial implementation of the `roadline` API which forms the basis for visual renderings of roadmaps and timelines.

- **Starts:** T1 + 1 day
- **Depends-on:** [T1](#t1-organization)
- **Ends:** T6 + 5 days
- **Contents:**
    - **[T6.1](#t61-produce-initial-roadline-api)**: Produce initial `roadline` API
    - **[T6.2](#t62-provide-demo-of-roadline-api-ideally-using-a-roadmap-from-one-of-the-ramate-robles-or-oac)**: Provide demo of `roadline` API, ideally using a roadmap from one of the Ramate, Robles, or OAC
    - **[T6.3](#t63-generate-specification-for-roadline-api)**: Generate specification for `roadline` API

**T6** pursues an initial implementation of the `roadline` API which forms the basis for visual renderings of roadmaps and timelines.

**T3** seeks to accomplish the following itemized objectives:

#### T6.1: Produce initial `roadline` API
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

The initial `roadline` API shall render documents formatting like this one into reasonable and visually appealing roadmpas.

#### T6.2: Provide demo of `roadline` API, ideally using a roadmap from one of the Ramate, Robles, or OAC
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Using a video formatted document such as this one, demonstrate the utility of the `roadline` API. Record a video.

#### T6.3: Generate specification for `roadline` API
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

Generate a specification, likely RASPEC, for `roadline`. We ask for to be done after the initial concept in this case to allow for a more informed specification to be generated.

## Agreeing
- **[AGR-1: Liam Monninger](./agreeing/agr-001-liam-monninger/README.md):** argues that this roadmap describes an effective plan for both procedural and psychological preparation ([Liam Monninger](mailto:liam@ramate.io)).

## Dissenting
$\emptyset$

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
