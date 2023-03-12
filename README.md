CSV-powered Rustlang Analyzer for Ballots
=========================================
This app will dynamically load a CSV, compute Ranked Choice Voting, and provide simple stats.

Short description of use
------------------------
1. Create a CSV with an ID column, then the ranked choices 1-to-N as the remaining columns
2. Build if needed the app `cargo build`
3. Run `--help` (to see if flags/commands changed)
4. Execute the application while passing if the CSV from #1
