An instance-only container with a guaranteed drop can make an outdoor collection look guaranteed too. Calculate object drop costs from providers with usable world locations, so collecting six items from a ten-percent outdoor container requires sixty uses even when an instance-only provider drops them every time.

Validation:

- The ordinary outdoor-quest fixture failed on upstream with six uses instead of sixty, then passed after the one-line source filter.
- A creature-drop control retains its original twenty-four kills for six items at twenty-five percent; both fixture regressions pass.
- Twelve existing item integration regressions, Rustfmt, whitespace checks and Clippy with warnings denied pass.
