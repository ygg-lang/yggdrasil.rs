


```shell
java -jar ./antlr4-4.8-2-SNAPSHOT-complete.jar -o ./src/antlr -encoding utf8 -listener -visitor -Dlanguage=Rust YggdrasilAntlr.g4; 
cargo fmt;
```