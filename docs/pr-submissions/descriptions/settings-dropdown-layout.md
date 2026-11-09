**AI disclosure:** This contribution was created with AI. The submitter reports in-game testing of the combined fork only and has not reviewed the source code. These split branches have not been individually verified in-game. Automated checks below were run by AI; no human source-code review is claimed.

Dropdown settings now use their available width and fit their options without a second reset control. The default remains a normal choice; changed sliders and switches keep their reset buttons.

AI-run automated checks: frontend build, both existing tests, oxlint and all 1166 translation checks pass. A React render probe fails against upstream's duplicate reset and passes with this change, checking default/current options and slider/switch resets. The build retains its existing large-chunk warning. The AI did not perform browser visual validation.

Part of the [22-topic contribution series](https://github.com/prijkes/factoruide/blob/pr/submission-plan/docs/pr-submissions/README.md).
