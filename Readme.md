# Readme

Turing Machine in Rust. Step by step.
Port of [ivanmoreau/turinghaskell](https://github.com/ivanmoreau/turinghaskell)

## Usage/Example

```bash
cargo run
```

## Example

```Haskell
testMachine :: Machine
```

```bash
*Main> print $ run testMachine ['1','1','1','1','1','1','1','1','0','1','1','1','1','1','1','1','1']
-- LONG VERBOSE OUTPUT --
-- ................... --
,_________111111111111111111111111111111111111111111111111111111 q_20 aaaaaaaaaabbbbbbbb_
,_________1111111111111111111111111111111111111111111111111111111 q_20 aaaaaaaaabbbbbbbb_
,_________11111111111111111111111111111111111111111111111111111111 q_20 aaaaaaaabbbbbbbb_
,_________111111111111111111111111111111111111111111111111111111111 q_20 aaaaaaabbbbbbbb_
,_________1111111111111111111111111111111111111111111111111111111111 q_20 aaaaaabbbbbbbb_
,_________11111111111111111111111111111111111111111111111111111111111 q_20 aaaaabbbbbbbb_
,_________111111111111111111111111111111111111111111111111111111111111 q_20 aaaabbbbbbbb_
,_________1111111111111111111111111111111111111111111111111111111111111 q_20 aaabbbbbbbb_
,_________11111111111111111111111111111111111111111111111111111111111111 q_20 aabbbbbbbb_
,_________111111111111111111111111111111111111111111111111111111111111111 q_20 abbbbbbbb_
,_________1111111111111111111111111111111111111111111111111111111111111111 q_20 bbbbbbbb_
,_________1111111111111111111111111111111111111111111111111111111111111111_ q_20 bbbbbbb_
,_________1111111111111111111111111111111111111111111111111111111111111111__ q_20 bbbbbb_
,_________1111111111111111111111111111111111111111111111111111111111111111___ q_20 bbbbb_
,_________1111111111111111111111111111111111111111111111111111111111111111____ q_20 bbbb_
,_________1111111111111111111111111111111111111111111111111111111111111111_____ q_20 bbb_
,_________1111111111111111111111111111111111111111111111111111111111111111______ q_20 bb_
,_________1111111111111111111111111111111111111111111111111111111111111111_______ q_20 b_
,_________1111111111111111111111111111111111111111111111111111111111111111________ q_20 _
,_________1111111111111111111111111111111111111111111111111111111111111111________ q_30 _
*Main> 
```

## Licence
GPL-v3