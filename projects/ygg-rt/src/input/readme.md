Provides trait constraints for parsing streaming input structures

## [`SequenceBuilder`]

Consider the following string injection:

```c
char* json5 = "{\n"
    "    a: 10\n"
    "}"
```

How should we parse this json and provide intelligent prompts?