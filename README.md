CSV-powered Rustlang Analyzer for Ballots
=========================================
This app will dynamically load a CSV, compute Ranked Choice Voting, and provide simple stats.

Short description of use
------------------------
1. Create a CSV with an ID column, then the ranked choices 1-to-N as the remaining columns
2. Build if needed the app `cargo build`
3. Run `--help` (to see if flags/commands changed)
4. Execute the application while passing if the CSV from #1

"Classic" RCV vs. Borda Count
-----------------------------

From ChatGPT:
```
Ranked Choice Borda and Ranked Choice Voting (RCV) are both methods for electing a candidate with
majority support. However, they use different algorithms and may produce different winners in
some cases.

In Ranked Choice Borda, each voter ranks the candidates in order of preference, and each
candidate is awarded points based on their ranking. The candidate with the most points wins the
election. The number of points awarded to a candidate for each ranking is determined by the 
Borda count method.

In RCV, voters rank the candidates in order of preference, and the candidate with the least
number of first-place votes is eliminated in each round. The votes for the eliminated candidate
are then transferred to the remaining candidates based on the voter's second choice. This
process is repeated until one candidate has a majority of the votes.

In some cases, the winner produced by RCV and Ranked Choice Borda may be the same. However, it is
also possible for the two methods to produce different winners, especially in elections with more
than two candidates.

For example, suppose there are three candidates: A, B, and C, and three voters with the following
preferences:
Voter 1: A > B > C
Voter 2: B > C > A
Voter 3: C > A > B

In Ranked Choice Borda, the candidates receive the following points:
A: 5 (1st place from Voter 1 + 3rd place from Voter 3)
B: 4 (2nd place from Voter 1 + 1st place from Voter 2)
C: 3 (3rd place from Voter 1 + 2nd place from Voter 2)

Therefore, A wins the election in Ranked Choice Borda.

In RCV, the first round of vote-counting looks like this:
A: 1 vote
B: 1 vote
C: 1 vote

Since no candidate has a majority of the votes, the candidate with the least number of
first-place votes (C) is eliminated. The votes for C are transferred to the remaining candidates
based on the voters' second choices:
Voter 1: A > B > C (transferred to B)
Voter 2: B > C > A (transferred to A)
Voter 3: C > A > B (eliminated)

The second round of vote-counting looks like this:
A: 2 votes
B: 1 vote

Since A has a majority of the votes, A wins the election in RCV.

Therefore, in this example, the winner produced by RCV and Ranked Choice Borda is different.
```
