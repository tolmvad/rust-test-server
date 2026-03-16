# Test command

# Check server run

```bash
curl <server>

curl <server> | jq
```

# Run benchmark

```bash
wrk -t 8 -c 100 -d 20s <server>
```

# first test result

```fisrt
wrk -t 8 -c 100 -d 20s http://0.0.0.0:3000

Running 20s test @ http://0.0.0.0:3000
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   840.17us  457.52us  16.05ms   88.32%
    Req/Sec    14.74k     1.99k   85.40k    95.94%
  2348593 requests in 20.10s, 291.17MB read
Requests/sec: 116845.69
Transfer/sec:     14.49MB
```

```second
Running 20s test @ http://127.0.0.1:3000
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency   849.87us  463.94us  16.18ms   88.63%
    Req/Sec    14.60k     1.01k   20.87k    83.06%
  2324126 requests in 20.02s, 394.53MB read
Requests/sec: 116110.20
Transfer/sec:     19.71MB

wrk -t 8 -c 100 -d 20s http://127.0.0.1:3000/json
Running 20s test @ http://127.0.0.1:3000/json
  8 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     0.95ms  495.87us  16.92ms   87.45%
    Req/Sec    12.90k     1.72k   69.08k    96.13%
  2055369 requests in 20.10s, 286.18MB read
Requests/sec: 102259.20
Transfer/sec:     14.24MB
```
