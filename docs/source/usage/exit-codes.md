# Best Practices

We generally try and give exit codes some meaning. We do not use a system where diffrent digits represent diffrent thigns. Common meaningful examples are recigniseable numbers or short 2--5 number sequences that spell something when typed on a telephone. The standard for that, if you are unfamilliar with it, is each number can mean 3 letters, which are as follows: 

```
 1    2    3
     abc  def

 4    5    6
ghi  jkl  mno

 7    8    9
pqrs tuv wxyz
```

As many exit code

`3467` DIMP

`0` : Unknown Exit Reason
`1` : Catch-All Exit
`2` : User Manual Exit
`123` : Failed to Iterate (Rationale: Iterators count, this counts)
`236` : Config Files Missing (Rationale: when typed on a telephone, this could be "CFM", or "Config Files Missing")
`255` : Max Packet Capacity Reached. This error code is 255 since it is easily recignised as an intiger limit, in the hopes that people may have an idea of what happened without looking at the docs.
`256` : Overflow Error (Rationale: `255` is the maximum that can be represented by a u8, so `256` overflows it.)
`863` : User Manual Exit (Rationale: when typed on a telephone, this could be "UME", or "User Manual Exit")