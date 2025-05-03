# OROAD-0: The Attempt
- **Authors:** [Liam Monninger](liam@ramate.io)

## Summary
**OROAD-0** is the foundational roadmap for OAC. It seeks to develop and validate a series of foundational papers, render a series of implementations from these papers, and the output applications demonstrating the utility of these papers. In the end, **OROAD-0** describes the series efforts which will be used to determine whether OAC is worth pursuing.  

The foundational papers anticipated by **OROAD-0** are: 
- **[OART-1: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-001-bfa/README.md):** describes a class of sampling protocols which accept Byzantine minority decisions with some non-zero probability; additionally formalizes the expected value of said decisions arguing for the ability for them to be rendered irrational. BFA are still deterministic and final.
- **[OART-2: Collaborative Transaction Routing](../../../oart/oera-000-000-000-dulan/oart-000-000-002-ctr/README.md):** describes a class of sortition-based transaction broadcast protocols. These allow incentivization to be shifted out of native token and into discretionary "super" protocols.
- **[OART-3: RIS-STM](../../../oart/oera-000-000-000-dulan/oart-000-000-003-ris-stm/README.md):** describes a generalization of [Block-STM](https://arxiv.org/abs/2203.06871) which plays forward best-case latency. 

The foundational implementations anticipated by **OROAD-0** are: 
- **[`gwrdfa`](https://github.com/ramate-io/gwrdfa):** an implementation of **[OART-1: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-001-bfa/README.md)** protocol substack. This forms the basis for high-throughput and large footprint OAC applications.
2. **[`srcavei`](https://github.com/ramate-io/srcavei):** an implementation of the **[OART-2: Collaborative Transaction Routing](../../../oart/oera-000-000-000-dulan/oart-000-000-002-ctr/README.md)** substack. This forms the basis for incentivization—which would no longer be strictly coin-based.
3. **[`fuste`](https://github.com/ramate-io/fuste):** a RISC-V VM with a set of adapters tailored to DLT—particularly plugging into the stack above. This is also critical to throughput and large footprint. 
4. **[`zhiye`](https://github.com/ramate-io/zhiye):** implementation of **[OART-3: RIS-STM](../../../oart/oera-000-000-000-dulan/oart-000-000-003-ris-stm/README.md)**. This takes advantage of some properties of BFA to greatly reduce best-case latency. 

All of these implementations are part of [Ramate LLC's](ramate.io) [`robles`](https://github.com/ramate-io/robles) stack. 

> [!TIP]
> **[[Liam Monninger](liam@ramate.io)]**
> 
> **OROAD-0** speculates much more into specific implementations and applications than I would expect later roadmaps to do. We justify this on the basis of the concept needing to initial validate itself and so needing to demonstrate end-goal utility. 
> 
> After **OROAD-0** our initial intent is to shift the planning done by OAC to the more conceptually focused--almost taking the stance of a journal or academic institution. 


## Roadmap
> [!WARNING]
> Ensure **All leads** contains list of all leads from milestones below.
> 
> **[AI Prompt]** help contributors to ensure the above. 

- **All leads:** [Liam Monninger](liam@ramate.io)
- **Contents:**
    - **[T1](#t1-push-towards-validation):** Push Towards Validation
    - **[T2](#t2-validation-and-accepting-contributions):** Validation and Accepting Contributions
    - **[T3](#t3-continued-validation-and-fuste-mvp):** Continued Validation and Fuste MVP
    - **[T4](#t4-exotic-execution):** Exotic Execution
    - **[T5](#t5-dlt-push):** DLT Push
    - **[T6](#t6-killer-apps-phase-1-traditional-l1):** Killer Apps Phase 1: Traditional L1
    - **[T7](#t7-killer-apps-phase-2-collaborative-streaming):** Killer Apps Phase 2: Collaborative Streaming
    - **[T8](#t8-the-decision-and-swarm-coordination):** The Decision and Swarm Coordination

### T1: Push Towards Validation
> [!IMPORTANT]
> **T1** focuses on readying OAC for validation. 

- **Starts:** T1 + 0 months
- **Depends-on:** $\emptyset$
- **Ends:** T1 + 1 month
- **Contents:**
    - **[T1.1](#t11-complete-draft-of-oart-1-bfa)**: Complete draft of [OART-1: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-001-bfa/README.md) 
    - **[T1.2](#t12-complete-draft-of-oart-2-collaborative-transaction-routing)**: Complete draft of [OART-2: Collaborative Transaction Routing](../../../oart/oera-000-000-000-dulan/oart-000-000-002-ctr/README.md)
    - **[T1.3](#t13-begin-gwrdfa-implementation)**: Begin [`gwrdfa`](https://github.com/ramate-io/gwrdfa) implementation
    - **[T1.4](#t14-begin-srcavei-implementation)**: Begin [`srcavei`](https://github.com/ramate-io/srcavei) implementation
    - **[T1.5](#t15-begin-fuste-implementation)**: Begin [`fuste`](https://github.com/ramate-io/fuste) implementation

**T1** features a push towards rendering content which will the initial validation of Ordered Atomic Collaboration (OAC). 

**T1** attempts to accomplish the following itemized intentions:

#### T1.1: Complete draft of [OART-1: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-001-bfa/README.md) 
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T1.2: Complete draft of [OART-2: Collaborative Transaction Routing](../../../oart/oera-000-000-000-dulan/oart-000-000-002-ctr/README.md)
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T1.3: Begin [`gwrdfa`](https://github.com/ramate-io/gwrdfa) implementation
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T1.4: Begin [`srcavei`](https://github.com/ramate-io/srcavei) implementation
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T1.5: Begin [`fuste`](https://github.com/ramate-io/fuste) implementation as lesser priority to [T1.3](#t13-begin-gwrdfa-implementation) and [T1.4](#t14-begin-srcavei-implementation)
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

### T2: Validation and Accepting Contributions
> [!IMPORTANT]
> **T2** focuses on beginning validation of OAC and adding contributors.

- **Starts:** T1 + 1 month
- **Depends-on:** [T1](#t1-push-towards-validation)
- **Ends:** T2 + 1 month
- **Contents:**
    - **[T2.1](#t21-share-and-gather-feedback-on-oart-1-bfa)**: Share and gather feedback on [OART-1: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-001-bfa/README.md)
    - **[T2.2](#t22-share-and-gather-feedback-on-oart-2-collaborative-transaction-routing)**: Share and gather feedback on [OART-2: Collaborative Transaction Routing](../../../oart/oera-000-000-000-dulan/oart-000-000-002-ctr/README.md)
    - **[T2.3](#t23-implement-and-document-proposal-standards)**: Implement and document proposal standards, contributor guidelines, and implementation governance
    - **[T2.4](#t24-complete-gwrdfa-reference-implementation)**: Complete [`gwrdfa`](https://github.com/ramate-io/gwrdfa) reference implementation
    - **[T2.5](#t25-complete-srcavei-reference-implementation)**: Complete [`srcavei`](https://github.com/ramate-io/srcavei) reference implementation
    - **[T2.6](#t26-continue-development-of-fuste)**: Continue development of [`fuste`](https://github.com/ramate-io/fuste) as a lower priority task

**T2** focuses on validating the initial content and establishing contribution frameworks for Ordered Atomic Collaboration (OAC) and [`robles`](https://github.com/ramate-io/robles).

**T2** attempts to accomplish the following itemized intentions:

#### T2.1: Share and gather feedback on [OART-1: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-001-bfa/README.md)
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)s

#### T2.2: Share and gather feedback on [OART-2: Collaborative Transaction Routing](../../../oart/oera-000-000-000-dulan/oart-000-000-002-ctr/README.md)
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T2.3: Implement and document proposal standards, contributor guidelines, and implementation governance
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T2.4: Complete [`gwrdfa`](https://github.com/ramate-io/gwrdfa) reference implementation
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T2.5: Complete [`srcavei`](https://github.com/ramate-io/srcavei) reference implementation
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T2.6: Continue development of [`fuste`](https://github.com/ramate-io/fuste) as a lower priority task
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

### T3: Continued Validation and [`fuste`](https://github.com/ramate-io/fuste) MVP
> [!IMPORTANT]
> **T3** continues validation and pushes for first proper application built with an OAC implementer's stack.

- **Starts:** T2 + 1 month
- **Depends-on:** [T2](#t2-validation-and-accepting-contributions)
- **Ends:** T3 + 1 month
- **Contents:**
    - **[T3.1](#t31-continue-sharing-and-updating-oart-1-bfa)**: Continue sharing and updating [OART-1: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-001-bfa/README.md)
    - **[T3.2](#t32-continue-sharing-and-updating-oart-2-collaborative-transaction-routing)**: Continue sharing and updating [OART-2: Collaborative Transaction Routing](../../../oart/oera-000-000-000-dulan/oart-000-000-002-ctr/README.md)
    - **[T3.3](#t33-develop-fuste-mvp)**: Develop [`fuste`](https://github.com/ramate-io/fuste) MVP
    - **[T3.4](#t34-use-fuste-mvp-to-develop-centralized-embedded-database)**: Use [`fuste`](https://github.com/ramate-io/fuste) MVP to develop centralized embedded database

**T3** focuses on continued validation of core concepts and the development of the Fuste MVP as a proof of concept for Ordered Atomic Collaboration (OAC).

**T3** attempts to accomplish the following itemized intentions:

#### T3.1: Continue sharing and updating [OART-1: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-001-bfa/README.md)
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T3.2: Continue sharing and updating [OART-2: Collaborative Transaction Routing](../../../oart/oera-000-000-000-dulan/oart-000-000-002-ctr/README.md)
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T3.3: Develop [`fuste`](https://github.com/ramate-io/fuste) MVP 
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T3.4: Use [`fuste`](https://github.com/ramate-io/fuste) MVP to develop centralized embedded database
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

### T4: Exotic Execution
> [!IMPORTANT]
> **T4** departs from the drive of previous milestones and takes some time to explore exotic distributed execution.

- **Starts:** T3 + 1 month
- **Depends-on:** [T3](#t3-continued-validation-and-fuste-mvp)
- **Ends:** T4 + 1 month
- **Contents:**
    - **[T4.1](#t41-draft-and-share-oart-3-ris-stm)**: Draft and share [OART-3: RIS-STM](../../../oart/oera-000-000-000-dulan/oart-000-000-003-ris-stm/README.md)
    - **[T4.2](#t42-collaborate-with-stanford-colleague)**: Collaborate with Stanford colleague working with Mary Wooters on exotic probabilistic distributed VM
    - **[T4.3](#t43-begin-zhiye-implementation)**: Begin [`zhiye`](https://github.com/ramate-io/zhiye) implementation

**T4** focuses on exploring exotic execution models and probabilistic distributed systems for Ordered Atomic Collaboration (OAC).

**T4** attempts to accomplish the following itemized intentions:

#### T4.1: Draft and share [OART-3: RIS-STM](../../../oart/oera-000-000-000-dulan/oart-000-000-003-ris-stm/README.md)
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T4.2: Collaborate with Stanford colleague working with Mary Wooters on exotic probabilistic distributed VM
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T4.3: Begin [`zhiye`](https://github.com/ramate-io/zhiye) implementation
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

### T5: DLT Push
> [!IMPORTANT]
> **T5** seeks to bring up stable implementations of OAC decentralization and prepare to use these to create a traditional DLT.

- **Starts:** T4 + 1 month
- **Depends-on:** [T4](#t4-exotic-execution)
- **Ends:** T5 + 1 month
- **Contents:**
    - **[T5.1](#t51-stabilize-gwrdfa-implementation)**: Stabilize [`gwrdfa`](https://github.com/ramate-io/gwrdfa) implementation
    - **[T5.2](#t52-stabilize-srcavei-implementation)**: Stabilize [`srcavei`](https://github.com/ramate-io/srcavei) implementation
    - **[T5.3](#t53-seek-out-additional-co-authors)**: Seek out additional co-authors for OAC foundational papers
    - **[T5.4](#t54-prepare-oart-1-bfa-for-conference)**: Prepare [OART-1: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-001-bfa/README.md) for conference submission
    - **[T5.5](#t55-prepare-oart-2-ctr-for-conference)**: Prepare [OART-2: Collaborative Transaction Routing](../../../oart/oera-000-000-000-dulan/oart-000-000-002-ctr/README.md) for conference submission
    - **[T5.6](#t56-begin-search-for-funding)**: Begin search for funding opportunities

**T5** focuses on stabilizing core implementations and preparing academic work for broader dissemination within the Ordered Atomic Collaboration (OAC) framework.

**T5** attempts to accomplish the following itemized intentions:

#### T5.1: Stabilize [`gwrdfa`](https://github.com/ramate-io/gwrdfa) implementation
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T5.2: Stabilize [`srcavei`](https://github.com/ramate-io/srcavei) implementation
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T5.3: Seek out additional co-authors for OAC foundational papers: [OART-1: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-001-bfa/README.md), [OART-2: Collaborative Transaction Routing](../../../oart/oera-000-000-000-dulan/oart-000-000-002-ctr/README.md), and [OART-3: RIS-STM](../../../oart/oera-000-000-000-dulan/oart-000-000-003-ris-stm/README.md)
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T5.4: Prepare [OART-1: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-001-bfa/README.md) for conference submission
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T5.5: Prepare [OART-2: Collaborative Transaction Routing](../../../oart/oera-000-000-000-dulan/oart-000-000-002-ctr/README.md) for conference submission
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T5.6: Begin search for funding opportunities
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

### T6: Killer Apps Phase 1: Traditional L1
> [!IMPORTANT]
> **T6** emphasizes the support of the first killer app built with OAC: a traditional L1 blockchain.

- **Starts:** T5 + 1 month
- **Depends-on:** [T5](#t5-dlt-push)
- **Ends:** T6 + 1 month
- **Contents:**
    - **[T6.1](#t61-design-and-implement-l1-applications)**: Design and implement traditional L1 blockchain applications using OAC principles
    - **[T6.2](#t62-integrate-core-implementations)**: Integrate [`gwrdfa`](https://github.com/ramate-io/gwrdfa) and [`srcavei`](https://github.com/ramate-io/srcavei) into L1 applications
    - **[T6.3](#t63-document-implementation-patterns)**: Document and share implementation patterns for OAC-based L1 applications
    - **[T6.4](#t64-gather-community-feedback)**: Begin gathering feedback from the broader blockchain community

**T6** focuses on developing traditional L1 blockchain applications within the Ordered Atomic Collaboration (OAC) framework.

**T6** attempts to accomplish the following itemized intentions:

#### T6.1: Design and implement traditional L1 blockchain applications using OAC principles
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T6.2: Integrate [`gwrdfa`](https://github.com/ramate-io/gwrdfa) and [`srcavei`](https://github.com/ramate-io/srcavei) into L1 applications
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T6.3: Document and share implementation patterns for OAC-based L1 applications
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T6.4: Begin gathering feedback from the broader blockchain community
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

### T7: Killer Apps Phase 2: Collaborative Streaming
> [!IMPORTANT]
> **T7** emphasizes the support of an collaborative streaming application built with OAC implementations.

- **Starts:** T6 + 1 month
- **Depends-on:** [T6](#t6-killer-apps-phase-1-traditional-l1)
- **Ends:** T7 + 1 month
- **Contents:**
    - **[T7.1](#t71-build-collaborative-streaming-platform)**: Begin building collaborative streaming platform using [`gwrdfa`](https://github.com/ramate-io/gwrdfa) and [`srcavei`](https://github.com/ramate-io/srcavei)
    - **[T7.2](#t72-seek-streaming-collaborators)**: Seek collaborators from traditional streaming and p2p content sharing communities
    - **[T7.3](#t73-guide-l1-demos)**: Guide and support L1 killer apps demonstration and deployment

**T7** focuses on developing collaborative streaming applications and expanding the OAC ecosystem through partnerships and demonstrations.

**T7** attempts to accomplish the following itemized intentions:

#### T7.1: Begin building collaborative streaming platform using [`gwrdfa`](https://github.com/ramate-io/gwrdfa) and [`srcavei`](https://github.com/ramate-io/srcavei)
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T7.2: Seek collaborators from traditional streaming and p2p content sharing communities
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T7.3: Guide and support L1 killer apps demonstration and deployment
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

### T8: The Decision and Swarm Coordination
> [!IMPORTANT]
> **T8** emphasizes completion of validation, OAC implementations of killer apps, and ultimately seeks to determine whether the project is worth continuing to pursue.

- **Starts:** T7 + 1 month
- **Depends-on:** [T7](#t7-killer-apps-phase-2-collaborative-streaming)
- **Ends:** T8 + 1 month
- **Contents:**
    - **[T8.1](#t81-guide-l1-demonstrations)**: Guide and support L1 blockchain applications demonstration
    - **[T8.2](#t82-guide-streaming-demonstrations)**: Guide and support Collaborative Streaming platform demonstration
    - **[T8.3](#t83-research-swarm-coordination)**: Research and experiment with swarm coordination mechanisms
    - **[T8.4](#t84-final-push-bfa)**: Final push for academic recognition of [OART-1: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-001-bfa/README.md)
    - **[T8.5](#t85-final-push-ctr)**: Final push for academic recognition of [OART-2: Collaborative Transaction Routing](../../../oart/oera-000-000-000-dulan/oart-000-000-002-ctr/README.md)
    - **[T8.6](#t86-final-push-ris-stm)**: Final push for academic recognition of [OART-3: RIS-STM](../../../oart/oera-000-000-000-dulan/oart-000-000-003-ris-stm/README.md)

**T8** focuses on finalizing demonstrations, exploring swarm coordination, and pushing for broader academic recognition of Ordered Atomic Collaboration (OAC).

**T8** attempts to accomplish the following itemized intentions:

#### T8.1: Guide and support L1 blockchain applications demonstration
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T8.2: Guide and support Collaborative Streaming platform demonstration
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T8.3: Research and experiment with swarm coordination mechanisms
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T8.4: Final push for academic recognition of [OART-1: BFA](../../../oart/oera-000-000-000-dulan/oart-000-000-001-bfa/README.md)
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T8.5: Final push for academic recognition of [OART-2: Collaborative Transaction Routing](../../../oart/oera-000-000-000-dulan/oart-000-000-002-ctr/README.md)
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)

#### T8.6: Final push for academic recognition of [OART-3: RIS-STM](../../../oart/oera-000-000-000-dulan/oart-000-000-003-ris-stm/README.md)
- **Lead:** [Liam Monninger](mailto:liam@ramate.io)