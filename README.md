# Baby Batch

## Overview

![alt text](BabyBatchCupcakeCrab.png "BabyBatch CupCake Crab :)")

Baby batch is a simple client cli written in rust. Currently, this iteration _only_ supports requests from the [YouTube Data API](https://developers.google.com/youtube/v3).

This cli was made using [replicate](https://github.com/ericdeansanchez/replicate) via:

```zsh
  $ replicate cli babybatch 
```

A small note: an initial request to the data api is made. A succesful
response contains YT's `nextPageToken`. Once we have this information,
we can make the next request. So, here at this point, we have a token
and some data.

The token remains in the current **thread** in order to initiate the
next request, whereas the data is passed to another thread to be processed
or passed off or both.

## Next Steps

- configure the cli semantics (i.e. `$ babybatch <uri> <outdir>`)
  - and then maybe something like `$ babybatch --json <uri> <outdir>`
  - the idea is to mimic a unix-y composition wherein usage might look like:

```text
$ babybatch --json <uri> <outdir> | crib --schema=stats --out=csv
```

- in the above, the output of `babybatch` is piped into the `crib` where
  it's nursed into some user defined schema<small><sup>1</sup></small> before
  being written out into the specified output format.

## Table of Contents

- [Baby Batch](#baby-batch)
  - [Overview](#overview)
  - [Next Steps](#next-steps)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [Contributing](#contributing)

## Installation

```text
  brew install `baby-batch **might** be coming soon`
```

## Contributing

Contributions are welcome! No contribution is too small––bug fix, a new feature,
or a typo fix––all are welcome.

- contribution [template]()
- issue [template]()


<small><sup>1</sup></small> This 'schema' can be viewed as a filter; it will
only accept the bits of information that adhere to the schema etc. etc.
