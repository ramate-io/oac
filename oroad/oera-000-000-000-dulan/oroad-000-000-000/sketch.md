There are effectively four novel modules in the stack I've been waiting to build, when the path clears. If Movement goes up in flames, this is what I'm going to take some time to do. 

The end objective is a high-performance and large footprint DLT. But, I'd aim to make this more like IETF-style protocols. I am personally most interested in swarm coordination, but this starts with general decentralized compute. 

(All names are working names.)

Modules in stack:
1. Robles: consensus core; implementation of BFA protocol substack (see below). This forms the basis for high-throughput and large footprint. 
2. Mali: collaborative transaction routing. This forms the basis for incentivization—which would no longer be strictly coin-based.
3. Quercia: a RISC-V VM with a set of adapters tailored to DLT—particularly plugging into the stack above. This is also critical to throughput and large footprint. 
4. Belot: generalization of Block STM scheduling for high-throughput. This takes advantage of some properties of BFA to greatly reduce best-case latency. 

Because some of the concepts are fundamentally new there are several papers I need to write that I've begun sketching.

Papers:
1. Byzantine Fault Allowing Protocols: describes and verifies a class of sampling protocols which accept Byzantine minority decisions with some non-zero probability but additional formalizes the expected value of said decisions arguing for the ability for them to be rendered irrational. BFA are still deterministic and final.
2. Collaborative Transaction Routing: describes and verifies a class of sortition-based transaction broadcast protocols. These allow incentivization to be shifted out of native token and into discretionary "super" protocols.
3. RIS-STM (Recursive Inclusion Set Software Transactional Memory): describes and verifies a generalization of Block-STM which plays forward best-case latency. 

I've planned for eight months before I go back to work. 

Month 1: Push towards Validation
- Complete drafts of BFA and Collaborative Transaction Routing.
- Begine Rust implementation of Robles and Mali reference implementations.
- Work on Quercia on the side. 

Month 2: Validation and Accepting Contributions
- Share BFA and Collaborative Transaction Routing paper drafts. 
- Implement proposal standards, contributor guidelines, implementation governance, etc. 
- Code work just to fill in gaps, continue progression. 
- Push to complete Robles and Mali reference implementations. 

Month 3: Continued Validation and Quercia MVP
- Continue to share and update BFA and Collaborative Transaction Routing papers.
- Push for Quercia MVP: just a centralized computing platform showing quercia usage. Planning to write this as an embedded database system. 

Month 4: Exotic Execution
- Draft RIS-STM paper and begin sharing.
- Collab with Stanford colleague working with Mary Wooters on exotic probabilistic distributed VM. 
- Begin on Belot implementation. 

Month 5: DLT Push
- Push for Robles and Mali stabilization. 
- Seek out additional co-authors. 
- Prepare BFA and Collaborative Transaction Routing papers for conferences. 
- Begin funding search. 

Month 6: Killer Apps Phase 1: Traditional L1
- Begin building incentivized L1 DLT with Robles and Mali. 
- Seek collaborators from blockchain space to propose high-throughput or large-footprint dApps to demonstrate DFA capabilities. 
- Seek contributors. 

Month 7: Killer Apps Phase 2: Collaborative Streaming
- Begin building collaborative streaming platform with Robles and Mali. 
- Seek collaborators from traditional streaming and p2p content sharing. 
- Guide L1 killer apps demo. 

Month 8: The Decision and Swarm Coordination
- Guide L1 and Collaborative Streaming demos.
- Mess around with swarm coordination. 
- Last push to gain traction on BFA, Collaborative Transaction Routing, and RIS-STM papers