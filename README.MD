# JsonTool

To convert a json file to a PS1 file run the following command:

```bash
cargo run -- --input=users.json --output=users.ps1 --mode=json2ps
```

To convert a PS1 file to a json file run the following command:

```bash
cargo run -- --input=users.ps1 --output=users.json --mode=ps2json
```
