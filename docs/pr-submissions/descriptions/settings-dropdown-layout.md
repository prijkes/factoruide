Dropdown settings now use their available width and fit their options without a second reset control. The default remains a normal choice; changed sliders and switches keep their reset buttons.

Validation: frontend build, both existing tests, oxlint and all 1166 translation checks pass. A React render probe fails against upstream's duplicate reset and passes with this change, checking default/current options and slider/switch resets. The build retains its existing large-chunk warning. Browser visual validation has not been performed.
