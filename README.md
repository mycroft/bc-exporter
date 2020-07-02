# bc-exporter

A very small bitcoin-core & altcoin prometheus exporter.

## Build

```sh
cargo build
```

## Run

```sh
cp bc-exporter.conf.sample bc-exporter.conf
./target/bc-exporter
```

## See metrics

```sh
curl http://localhost:32221/metrics
# HELP block_count Block Count
# TYPE block_count counter
block_count{name="bitcoin",node="http://localhost:8332",chaintype="bitcoin",test="false"} 637391
block_count{name="bitcoin-testnet",node="http://localhost:18332",chaintype="bitcoin",test="true"} 1774996
block_count{name="bitcoin-abc",node="http://localhost:8432",chaintype="bitcoin-abc",test="false"} 642196
block_count{name="bitcoin-sv",node="http://localhost:8532",chaintype="bitcoin-sv",test="false"} 641969
block_count{name="litecoin",node="http://localhost:9556",chaintype="litecoin",test="false"} 1869999
block_count{name="litecoin-testnet",node="http://localhost:19556",chaintype="litecoin",test="true"} 1542908
block_count{name="dogecoin",node="http://localhost:8156",chaintype="dogecoin",test="false"} 3296555
block_count{name="dash",node="http://localhost:8256",chaintype="dash",test="false"} 1297724
block_count{name="zcash",node="http://localhost:8356",chaintype="zcash",test="false"} 887456
```
