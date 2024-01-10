io benchmarks for python(django), rust and go. they both have a test endpoint, which gets a value from redis, randomly for 5% of the requests it takes 3 seconds more than the original time.

**For Python**->
**without gevent**
Running via `gunicorn abcd.wsgi --name=abcd --bind=0.0.0.0:8000 --workers 4 --thread 8`. The processes were getting killed and a single worker peak memory was 190M.
load testing via
`echo "GET http://localhost:8000/test" | vegeta attack -duration=15s --rate 800 | tee gunicorn.bin | vegeta report| less`
Results->
```
Requests      [total, rate, throughput]  12000, 800.08, 90.56
Duration      [total, attack, wait]      40.85534s, 14.998526875s, 25.856813125s
Latencies     [mean, 50, 95, 99, max]    12.595706038s, 13.123478067s, 21.962320733s, 25.912636259s, 30.0030025s
Bytes In      [total, mean]              526349, 43.86
Bytes Out     [total, mean]              0, 0.00
Success       [ratio]                    30.83%
Status Codes  [code:count]               0:8296  200:3700  500:4 
```

**with gevent**
Running via `gunicorn abcd.wsgi --name=abcd --bind=0.0.0.0:8001 --worker-class=gevent --workers 2`
`echo "GET http://localhost:8001/test" | vegeta attack -duration=15s --rate 800 | tee gevents.bin | vegeta report| less`. Peak memory was 140 MB for both the workers and CPU was 70%
```
Requests      [total, rate, throughput]  12000, 800.07, 666.56
Duration      [total, attack, wait]      18.003012458s, 14.998707333s, 3.004305125s
Latencies     [mean, 50, 95, 99, max]    150.726965ms, 1.990299ms, 852.732997ms, 3.006562943s, 3.02695375s
Bytes In      [total, mean]              132000, 11.00
Bytes Out     [total, mean]              0, 0.00
Success       [ratio]                    100.00%
Status Codes  [code:count]               200:12000 
```

**For Go**->
Running via `go build -o io_bench -ldflags="-s -w" .; ./io_bench`. Peak CPU was 17%, memory 18mb.
load testing via
`echo "GET http://localhost:8000/test" | vegeta attack -duration=15s --rate 800 | tee go.bin | vegeta report| less`
Results->
```
Requests      [total, rate, throughput]  12000, 800.09, 667.65
Duration      [total, attack, wait]      17.973621333s, 14.998283708s, 2.975337625s
Latencies     [mean, 50, 95, 99, max]    152.497182ms, 1.051542ms, 1.660912912s, 3.006845578s, 3.023703708s
Bytes In      [total, mean]              108000, 9.00
Bytes Out     [total, mean]              0, 0.00
Success       [ratio]                    100.00%
Status Codes  [code:count]               200:12000 
```

**For rust**->
Running via `cargo run --release`. Peak CPU was 8%, memory 10.4 mb.
```
Requests      [total, rate, throughput]  12000, 800.07, 666.99
Duration      [total, attack, wait]      17.991292583s, 14.998692541s, 2.992600042s
Latencies     [mean, 50, 95, 99, max]    157.255745ms, 1.306376ms, 2.12021904s, 3.006202906s, 3.020730666s
Bytes In      [total, mean]              108000, 9.00
Bytes Out     [total, mean]              0, 0.00
Success       [ratio]                    100.00%
Status Codes  [code:count]               200:12000
```