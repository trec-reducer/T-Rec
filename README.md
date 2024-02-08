# T-Rec
## Commands to run T-Rec
### Run Perse+T-Rec
```
java -jar t-rec.jar \
    --test-script <path to the test script> \
    --input-file <path to the file to be reduced> \
    --output-dir <path to the desired output directory>
    --query-caching true \
    --query-caching-type CONTENT_SHA512 \
    --on-demand-reducer-classes "org.perses.reduction.reducer.TokenCanonicalizer"
```

### Run Vulcan+T-Rec
```
java -jar t-rec.jar \
    --test-script <path to the test script> \
    --input-file <path to the file to be reduced> \
    --output-dir <path to the desired output directory>
    --enable-vulcan true \
    --query-caching true \
    --query-caching-type CONTENT_SHA512 \
    --on-demand-reducer-classes "org.perses.reduction.reducer.TokenCanonicalizer"
```
### Check all the available command line arguments
```
java -jar t-rec.jar --help
```
## Benchmarks
To run T-Rec with the benchmarks mentioned in the paper, follow in the instruction of the artifact of Vulcan, and replace jar file of vulcan with trec.jar.
Some rust benchmarks are not included in Vulcan's artifact, we include the completed rust benchmarks in this repository.
Link to the artifact of Vulcan: https://zenodo.org/records/8149017