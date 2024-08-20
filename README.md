# Lepton Server Benchmark

Benchmark of lepton decoding and serve with [lepton_jpeg](https://github.com/microsoft/lepton_jpeg_rust) library.

## Instructions

1. Put image into `images` dir
2. `docker compose up`
3. `ab -n 500 -c 10 http://localhost:8080/?i=your-lepton-image.lep`

In default, port `8080` is PHP and `8081` for Golang.

## Results

Only 1 image's result. Should try in your environment and image.

Processing threads are all 1. Resolution is 3508x2480.

### PHP

```
$ ab -n 500 -c 10 http://localhost:8080/?i=raw.lep
This is ApacheBench, Version 2.3 <$Revision: 1913912 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 100 requests
Completed 200 requests
Completed 300 requests
Completed 400 requests
Completed 500 requests
Finished 500 requests


Server Software:        Apache/2.4.61
Server Hostname:        localhost
Server Port:            8080

Document Path:          /?i=raw.lep
Document Length:        2612717 bytes

Concurrency Level:      10
Time taken for tests:   57.808 seconds
Complete requests:      500
Failed requests:        0
Total transferred:      1306438000 bytes
HTML transferred:       1306358500 bytes
Requests per second:    8.65 [#/sec] (mean)
Time per request:       1156.156 [ms] (mean)
Time per request:       115.616 [ms] (mean, across all concurrent requests)
Transfer rate:          22070.00 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    0   0.3      0       4
Processing:   461 1143 215.4   1110    2081
Waiting:      450 1121 209.1   1088    2047
Total:        461 1143 215.5   1110    2081

Percentage of the requests served within a certain time (ms)
  50%   1110
  66%   1182
  75%   1232
  80%   1284
  90%   1411
  95%   1624
  98%   1744
  99%   1838
 100%   2081 (longest request)
```

### Go

```
$ ab -n 500 -c 10 http://localhost:8081/?i=raw.lep
This is ApacheBench, Version 2.3 <$Revision: 1913912 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 100 requests
Completed 200 requests
Completed 300 requests
Completed 400 requests
Completed 500 requests
Finished 500 requests


Server Software:
Server Hostname:        localhost
Server Port:            8081

Document Path:          /?i=raw.lep
Document Length:        2612717 bytes

Concurrency Level:      10
Time taken for tests:   39.612 seconds
Complete requests:      500
Failed requests:        0
Total transferred:      1306399500 bytes
HTML transferred:       1306358500 bytes
Requests per second:    12.62 [#/sec] (mean)
Time per request:       792.240 [ms] (mean)
Time per request:       79.224 [ms] (mean, across all concurrent requests)
Transfer rate:          32206.92 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    0   0.1      0       1
Processing:   210  786 193.9    790    1382
Waiting:      201  765 189.0    772    1309
Total:        210  786 193.9    790    1382

Percentage of the requests served within a certain time (ms)
  50%    790
  66%    862
  75%    912
  80%    946
  90%   1037
  95%   1128
  98%   1214
  99%   1261
 100%   1382 (longest request)
```

### Rust

```
$ ab -n 500 -c 10 http://localhost:8082/?i=raw.lep
This is ApacheBench, Version 2.3 <$Revision: 1913912 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 100 requests
Completed 200 requests
Completed 300 requests
Completed 400 requests
Completed 500 requests
Finished 500 requests


Server Software:
Server Hostname:        localhost
Server Port:            8082

Document Path:          /?i=raw.lep
Document Length:        2612717 bytes

Concurrency Level:      10
Time taken for tests:   50.308 seconds
Complete requests:      500
Failed requests:        0
Total transferred:      1306412000 bytes
HTML transferred:       1306358500 bytes
Requests per second:    9.94 [#/sec] (mean)
Time per request:       1006.169 [ms] (mean)
Time per request:       100.617 [ms] (mean, across all concurrent requests)
Transfer rate:          25359.42 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    0   0.1      0       1
Processing:   536  992 257.8    901    2112
Waiting:      501  940 187.6    873    1686
Total:        536  992 257.8    901    2112

Percentage of the requests served within a certain time (ms)
  50%    901
  66%    993
  75%   1096
  80%   1147
  90%   1306
  95%   1542
  98%   1869
  99%   2026
 100%   2112 (longest request)
```
