```math
\begin{aligned}
BFT &\subset BFA \\
\alpha \cdot Loss(BFT) + \epsilon &\geq Loss(BFA)
\end{aligned}
```

```math
\begin{aligned}
B(CTR) &\subset AB \\
\\
\prod^{k} P[B(CTR_{BFA}(\zeta)) = 0] &\leq \mu \\
\\
E[Loss(BFA)] \leq \mu \cdot Loss(BFA) &\\
\quad \quad \rightarrow U(B(CTR_{BFA}(\zeta)) = 1) \gt U(\mathcal{F}) &
\end{aligned}
```

```math
\begin{aligned}
&\text{RIS-STM}(i', B, C): \\
&\quad \textbf{loop:} \\
&\quad\quad \textbf{for } i \in C_i: \\
&\quad\quad\quad C_i := C_i + \text{recv}(i, B) \\
&\quad\quad\quad \textbf{for } C_{i,k} \in C_i: \\
&\quad\quad\quad\quad C_{i',k} := \text{compute}(i', C_{i,k}) \\
&\quad\quad\quad\quad \textbf{if } C_{i,k} \in \text{FIN:} \\
&\quad\quad\quad\quad\quad \textbf{return } C_{i',k}
\end{aligned}
```

```math
\begin{aligned}
\text{Let } \mathcal{N} \text{ be the set of participants.} \\
\text{Let } \mathcal{H} \subseteq \mathcal{N} \text{ be the subset of honest participants.} \\
\text{Let } \mathcal{H} \subset \mathcal{F} \text{ be the subset of faulty participants.} \\

\text{We make the Byzantine assumption:} \\
|\mathcal{N}| = 3n + 1 \\
|\mathcal{H}| = 2n + 1 \\
|\mathcal{F}| = n \\

\text{Let } \mathcal{C} \text{ be the set of participants included in a subcommittee. We parameterize this subcommittee by } k: \\

|C| = 3k + 1 \\

\text{The total number of ways to select the subcommittee is:} \\

c = \binom{3n + 1}{3k + 1} \\

\text{We define an accepted faulty subcommittee as one where:} \\

|C \cap \mathcal{F}| >= 2k + 1 \\

\text{The total number of ways to select an accepted faulty subcommittee is:} \\

c'(n, k) = \sum_{h = 2k + 1}^{\min(3k + 1, n)} \binom{n}{h} \cdot \binom{2n + 1}{3k + 1 - h} \\

\text{The probability of accepting a faulty subcommittee is: } \\

Pr[Accepted Faulty] = c'(n,k)/c

\text{We define an accepted honest subcommittee as one where:} \\

|C \cap \mathcal{H}| >= 2k + 1 \\

\text{The total number of ways to select an accepted honest subcommittee is:} \\

c'(n, k) = \sum_{h = 2k + 1}^{\min(3k + 1, 2n + 1)} \binom{n}{h} \cdot \binom{n}{3k + 1 - h} \\
\end{aligned}
```

```math
\begin{aligned}
&& \lim_{n \to \infty} Pr[\text{Accepted Honest}](n, k) \\
&& \quad = 2^{-1} \forall k \in \mathbb{N} \\
&& \Rightarrow \Theta(BFA) \\
&& \quad \approx (Pr[\text{Accepted Honest}] + Pr[\text{Accepted Faulty}])k \\
&& \quad \quad + (1 - (Pr[\text{Accepted Honest}] + Pr[\text{Accepted Faulty}]))(n + k)  \\
&& \quad \approx \frac{k}{2} + \frac{n + k}{2} \\
&& \quad = \frac{n}{2} + k \\

&& \Omega(BFA) = k \\
\end{aligned}
```

```math
\begin{aligned}
\Theta(BFA) & \\
& = \sum_{n = 0}^{\infty} 2^{-n}k \\
& = 2k \\
\end{aligned}
```

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
  </sub>
</div>
