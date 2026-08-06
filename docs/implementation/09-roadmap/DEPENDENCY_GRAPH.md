# WAVE dependency graph

```text
W000
  |
W001
  |
W002
  |\
  | W004
  | /
W003
  \ /
 W005
   |
 W006
   |
 W007
   |
 W008
   |
 W009
   |
 W010
   |
 W011
   |
 W012
   |\
   | W013
   |  |
   +--+--W014
          |
        W015
          |
        W016
          |
        W017
          |
        W018
         / \
      W019 W020
         \ /
        W021
          |
        W022
          |
        W023
          |
        W024
```

## Parallelism rules

- W003 and W004 may proceed in parallel after W002.
- W019 and W020 may proceed in parallel only after W018 closes Gate S5.
- No Researcher source connector starts before W017 and W018.
- No host adapter starts before the activation security boundary in W007.
- No public release work starts while a security gate has unresolved critical or high findings.
