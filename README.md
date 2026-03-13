# Test command

```bash
wrk -t 8 -c 100 -d 20s <server>
```

# first test result

```info
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
