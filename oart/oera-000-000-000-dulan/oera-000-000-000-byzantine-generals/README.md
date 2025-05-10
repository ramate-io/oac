```math
\begin{aligned}
BFT &\subset BFA \\
\alpha \cdot Loss(BFT) + \epsilon &\geq Loss(BFA)
\end{aligned}
```

```math
\begin{aligned}
B(CTR) &\subset AB \\
\prod^{k} P[B(CTR_{BFA}(\zeta)) = 0] &\leq \mu \\
E[Loss(BFA)] \leq \mu \cdot Loss(BFA) &\\
\rightarrow U(B(CTR_{BFA}(\zeta)) = 1) &\gt U(\mathcal{F})
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
