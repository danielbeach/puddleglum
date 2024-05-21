## PuddleGlum

This is a Rust based Python package that allows for complex Data Quality 
checks on AWS `s3` buckets. 

`PuddleGlum` makes it easy to find out things like ...
- when was the last time a file was recieved?
- what was the last modified file?
- How large was the last file?
- How many files have recieved in the last `n` minutes, hours, days, weeks?

It's common for many Data Teams to store and recieved large amounts of 
raw files in `s3` buckets. Many times understanding what is happening
in these `s3` buckets is a critical first "line of defence" and Data Quality
checks.
