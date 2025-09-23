# fcheck
Cool tool for checking enabled and disabled features of a specified crate within a cargo(Rust) project

## Installation
`
cargo install fcheck
`
## Uses
within a cargo project type
`
fcheck <Name of a crate added to the project>
`
## example
command:
`
 fcheck serde
`

Output:
```
serde:
├── Enabled:
│   ├── default
│   ├── derive
│   ├── serde_derive
│   ├── std
└── Disabled:
    ├── alloc
    ├── rc
    ├── unstable
```





### contact:
<a href="https://linkedin.com/in/urparvezali" target="_blank">LinkedIn - urparvezali</a>
<br/>
<a href="mailto:urparvezali@gmail.com">Email - Click</a>
