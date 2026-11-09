A web app built on Windows silently omitted its game databases and map images because URL.pathname retained the drive-leading slash. Resolve the data URL with fileURLToPath so filesystem checks and copies use a native path.

Validation: a controlled Windows fixture in a path containing spaces reproduced the missing database before the fix and copied all three databases, map images and host headers after it; Node syntax and Git whitespace checks passed.
