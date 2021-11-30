



```
def token Identifier {
    [_0-9a-zA-Z] 
}
```



```
@position
Identifier = identifier:IDENTIFIER;

@string
@no_skip_ws
IDENTIFIER = 'a'..'z' | 'A'..'Z' | '_' | '0'..'9';
```



```
def string EOS {
    ';;' | ';'
}
```

```
@string
@no_skip_ws
EOS = ';;' | ';';
```