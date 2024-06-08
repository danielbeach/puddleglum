## PuddleGlum

<img src="https://github.com/danielbeach/puddleglum/blob/main/imgs/puddleglum.webp" width="450">


This is a `Rust`-based `Python` package that allows for complex Data Quality 
checks on AWS `s3` buckets. 

`pip install puddleglum`

`PuddleGlum` makes it easy to find out things like ...
- when was the last time a file was received?
- what was the last modified file?
- How large was the last file?
- How many files have received in the last `24` or `48` hours?

It's common for many Data Teams to store and received large amounts of 
raw files in `s3` buckets. Many times understanding what is happening
in these `s3` buckets is a critical first "line of defense" and Data Quality
checks.


#### Development
If you want to develop on this codebase, both Rust and Python and
in the provided `Dockerfile`.

To build it ... `docker build . --tag=puddles`
To drop into the image `docker run --volume ./app -it puddles /bin/bash`

##### Python
This project is ported to a Python package called `puddleglum` via `pyo3` and `maturin`
Use `maturin build` or `maturin develop` to build and/or drop into `venv` using this package.
To make a venv do `virtualenv -p python3 .` and then `source bin/activate` And the `maturin` command should
work from there.

`pip install puddleglum`

Usage
```
from puddleglum import S3

puddles = S3('some-bucket', 'some-prefix')
file = puddles.get_most_recent_file()
```
