# dist/

`index.html` is a byte-identical copy of the live Pocket Sheets app file
(`../pocket-sheets.html`, currently v1.1). It is generated, never edited by
hand. To regenerate after the live file changes:

```sh
cp ../pocket-sheets.html index.html
sha256sum ../pocket-sheets.html index.html   # the two hashes must match
```
