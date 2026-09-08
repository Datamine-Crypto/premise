# Reason corpus

70 `because!` reasons written by five authors across many rounds of building
features on Premise. Preserved verbatim, including phrasings that would normally be
tidied. Nothing here was written to pass or fail any rule.

This is spec content only. No app code, no tests, no reducers beyond the vocabularies
the reasons attach to. It is data, not a build target.

## Why it exists

Every other reason the reason-checks can be measured against was written by someone
who knew the rule. These were not. This is the only sample of the rule's actual input
distribution.

## What the author knew, per directory

| directory | reasons | author's knowledge of E-FALSE-RELATION when written |
|---|---|---|
| `ratelimit/` | 2 | rule did not exist |
| `inventory/` | 6 | rule did not exist |
| `loot/` | 4 | rule did not exist |
| `shipping/` | 5 | rule did not exist |
| `order/` | 9 | rule did not exist |
| `subscription/` | 10 | told it existed; had never seen it fire; did not know what forms it parsed |
| `lending/` | 5 | same as subscription |
| `garage/` | 7 | different author, building a parking garage; the rules were deleted before this was written |
| `bike/` | 9 | same author, e-bike hire scheme with two contexts and a superseded tariff |
| `lib3/` | 9 | same author, three-context library system |
| `holdqueue/` | 4 | an author with NO session history, given only the manual, building a hold queue |

70 reasons from five independent authors. 26 of the original 41 were written before the
reason-checking rules existed at all, and the 29 added later were written after those rules were
deleted, so no reason here was written to satisfy a checker that reads prose. That is the property
that makes this corpus worth keeping: it is a sample of what people actually write when nothing is
grading the writing.

The `holdqueue/` set is the most valuable and the smallest. Its author had never seen this system
before, read only the manual, and did not know a corpus existed.

## The claim this corpus does NOT support

It was reported that E-FALSE-RELATION caught something in the wild, on a live spec,
during ordinary work. That is not what happened, and the sequence matters.

1. `ANNUAL_CENTS = 12000` was written with the reason preserved here verbatim:
   "ANNUAL_LIST_CENTS less two months, the discount sales needs to close annual deals".
   ANNUAL_LIST_CENTS is 14400 and two months is 2400, so the claim is TRUE.
   The checker was silent. Correct, but uninformative: silence is also what an
   unparsed reason produces.

2. The value was changed to 13000, making the same claim FALSE. The reason was left
   untouched. The checker was still silent. The natural phrasing was never read.

3. The reason was then rewritten specifically into the rule's documented form,
   "MONTHLY_CENTS multiplied by MONTHS_PER_YEAR at the list rate with no discount",
   against the same broken value. Only then did E-FALSE-RELATION fire, and its
   message was excellent.

So the observed catch came from a phrasing constructed to test the rule, against a
value deliberately broken to test it. It is a successful probe. It is not a field
catch, and the corpus exists partly so that distinction stops being lost.

## The measurement worth running

By the author's own reading, 1 of these 41 reasons (`ANNUAL_CENTS`) names two
constants joined by an operator word, which is the form the rule can parse. The
checker did not read even that one. The other 40 are provenance rather than
arithmetic: who decided this, on what evidence, and what breaks if it changes.

That is the shape of the finding to verify: not a false-positive rate, but a
coverage rate. If it holds, E-FALSE-RELATION is sound and narrow, and its silence
carries almost no information on real domain prose. That is a fine thing for a rule
to be, provided nobody reads the silence as verification.

`E-WEAK-REASON` has a much larger applicable surface here, since it applies to all 41,
and is the more useful rule to measure against this corpus.

## Quoting the number

Five of the 41 are near-identical `SEPARATOR` reasons and two `STAGE_LABEL` reasons are
byte-identical, so the effective sample is smaller than 41 for anything measuring phrasing
diversity. That is a property of real specs rather than a flaw in the corpus, and it should
be said whenever the count is quoted.

Exactly one of the 41 makes an arithmetic claim, and it is phrased in units rather than
constant names, so `E-FALSE-RELATION` cannot read it either way. Its claim is true, so
silence is the correct outcome, but the rule is silent because it did not parse rather than
because it checked.
