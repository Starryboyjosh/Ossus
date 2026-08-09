# WAVE-020 — External analysis integrations

## Status

Planned. No repository evidence shows implementation or completion.

## Technical summary

Integrate mature scanners in disposable, bounded jobs: Semgrep, dependency and license adapters, timeout/resource limits, version capture, and normalized findings. Scanner output cannot alter canonical state, and a scanner failure is not a clean result.

## Practical plain-language summary

Run established safety and license checks on candidates in tightly controlled jobs, without giving their output authority to approve anything.

## Expected evidence/deliverables

- Experimental analyze command, scanner interfaces, disposable-runner documentation, and attributed support evidence.
- Tests for absent scanners, timeouts, malicious output, oversized findings, version mismatch, and disabled networking.
- Agent Review Authority security assessment.

## Dependencies/gates

Depends on WAVE-018. Together with WAVE-019, it is required before WAVE-021; it does not itself close a named gate.

## Remaining work

Implement adapters and bounded runner behavior, prove findings cannot mutate canonical state, test failures and hostile output, and obtain Agent Review Authority assessment.
