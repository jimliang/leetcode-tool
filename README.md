
# leetcode-tool

Leetcode tools for fetch Rust code and submit.

## Install

```bash
cargo install --git https://github.com/jimliang/leetcode-tool
```

## Usage

### Fetch
For example, fetch code from `https://leetcode.cn/problems/random-pick-with-blacklist/`, just

```bash
leetcode-tool fetch random-pick-with-blacklist
```

will generate file `src/random_pick_with_blacklist.rs` and add mod in `src/lib.rs`

### Submit

```bash
export COOKIE=<leetcode cookie>
leetcode-tool submit random-pick-with-blacklist
```

Auto submit you code and add code to git.