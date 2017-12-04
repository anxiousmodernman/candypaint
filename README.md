# candypaint

Candy coated prompts for the ion shell. Tested on Linux only, for now.

## Installation

Use cargo. Note that ~/.cargo/bin must be on your PATH

```
cargo install --git https://github.com/anxiousmodernman/candypaint
```

Then, add the following to **~/.config/ion/initrc**

```sh
fn PROMPT
    echo -n $(candypaint)
end
```

## Goals

This project aims to provide zero-config prompts for ion. Prompt modifications
might get deeper integration into ion in the future, but until then we can use
a dedicated tool to shell out to in our initrc.

