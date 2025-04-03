# fcheck
tool for checking enabled and disabled features of a specified crate within a cargo project

## Installation
`
cargo install fcheck --locked
`
## Uses
within a cargo project type
`
fcheck <Name of a crate added to the project>
`
## example
`
command: fcheck serde
`

Output:
```
Enabled: ["alloc", "default", "derive", "rc", "serde_derive", "std"]
Disabled: ["unstable"]
```