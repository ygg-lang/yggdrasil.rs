# interleave
Insert a symbol alternately

```ygg
@interleave(rule, separator, trailing)
- rule:      Expression #Untagged
- separator: Expression #Untagged
- trailing?: Integer
```

- `rule`:
- `separator`:
- `trailing`:

[a, b, c,]
[a, b, c]
[a, b, c,]

# binary_left
Embed images in documents
- smart link

```notedown
[some/path.png]
```

- inline mode

```notedown
\img: something will not shown`
```

- text mode

```notedown
\img[some tips not shown]
```

# binary_right
Link

todo

# unary_prefix
Omit the following text

**Only works in blog mode**

```notedown
\cmd: read_more
```


# unary_suffix
Table of Contents


```notedown
\toc
```


# hook
Ignore next header

todo

```notedown
\toc_ignore

## The header will not appear in TOC
```
