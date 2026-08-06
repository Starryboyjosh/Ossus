# Opus 5 — pre-implementation security and architecture review

You are the mandatory security and architecture reviewer for Ossus WAVE-000.

Review the full package independently.

Questions:

- Is the trusted computing base small enough?
- Are canonical and origin metadata truly separated?
- Can untrusted content influence scoring or policy?
- Does index-first distribution create unresolved fetch or license risk?
- Is activation transactional and race-resistant?
- Are host compatibility claims precise?
- Does the Rust crate graph preserve trust boundaries?
- Can private source precedence enable impersonation?
- Are security WAVEs placed before dependent functionality?
- Can the golden suite detect unsafe false activations?
- Is the Researcher still capable of contaminating trusted CI?

Output structured findings with evidence, severity, proposed plan change and test requirement.

Identify any evidence prepared by Luna Max or another implementation agent. Treat it as attributed support, verify it independently against the repository, and do not let it replace your own security judgment.

Conclude with BLOCK, REVISE, or READY FOR HUMAN DECISION.

Explicitly list where your review is uncertain.
