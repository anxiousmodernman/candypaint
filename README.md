# candypaint

Prompts for the ion shell. Tested on Linux only.

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


