What I Did
For this task, I worked on the scanner for the Kobo language. I completed the missing parts of the scanner so it can read the source code and turn it into tokens like numbers, identifiers, keywords, operators, and strings.
I also added handling for comments, spaces, new lines, and invalid characters.
What I Learned
I learned how a scanner reads a program one character at a time. I also learned how peek() and peek_next() can be used to check characters without moving forward.
I learned how the scanner can tell the difference between things like = and ==, or < and <=.
Problems I Faced
One thing I found confusing was handling strings that go across multiple lines. I had to make sure the line number increased when a new line was found.
Numbers were also a little tricky because a dot should only be included when there is a number after it.
How I Solved Them
I used separate functions for strings, numbers, and identifiers to make the scanner easier to handle.
I used matches() for operators that can have an = after them. I also used error() so that when an invalid character or unfinished string is found, the scanner can report it and continue.
What I Would Improve
If I did this task again, I would test more cases before finishing. I would test things like empty input, comments, decimal numbers, unfinished strings, and invalid characters.
Overall, this task helped me understand how a scanner works and gave me more practice with Rust functions, loops, conditions, and character handling.

