# Lepton Server Benchmark

Benchmark of lepton decoding and serve with [lepton_jpeg](https://github.com/microsoft/lepton_jpeg_rust) library.

## Prepare

1. Put image into `images` dir
2. `docker compose up`
3. `ab -n 3000 -c 100 http://localhost:8080/?i=your-lepton-image.lep`

In default, port `8080` is PHP and `8081` for Golang.

## Results

Only 1 image's result. Should try in your environment and image.

Processing threads are all 1. Resolution is 3508x2480.

### PHP

```
$ ab -n 3000 -c 100 http://localhost:8080/?i=raw.lep
This is ApacheBench, Version 2.3 <$Revision: 1913912 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 300 requests
Completed 600 requests
Completed 900 requests
Completed 1200 requests
Completed 1500 requests
Completed 1800 requests
Completed 2100 requests
Completed 2400 requests
Completed 2700 requests
Completed 3000 requests
Finished 3000 requests


Server Software:        Apache/2.4.61
Server Hostname:        localhost
Server Port:            8080

Document Path:          /?i=raw.lep
Document Length:        2612717 bytes

Concurrency Level:      100
Time taken for tests:   299.202 seconds
Complete requests:      3000
Failed requests:        0
Total transferred:      7838628000 bytes
HTML transferred:       7838151000 bytes
Requests per second:    10.03 [#/sec] (mean)
Time per request:       9973.388 [ms] (mean)
Time per request:       99.734 [ms] (mean, across all concurrent requests)
Transfer rate:          25584.45 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    0   0.3      0       4
Processing:   470 9888 2441.4  10044   24009
Waiting:      451 9874 2440.9  10030   24003
Total:        470 9888 2441.4  10044   24009

Percentage of the requests served within a certain time (ms)
  50%  10044
  66%  10564
  75%  10841
  80%  11052
  90%  11448
  95%  11920
  98%  17757
  99%  20974
 100%  24009 (longest request)
```

### Go

```
$ ab -n 3000 -c 100 http://localhost:8081/?i=raw.lep
This is ApacheBench, Version 2.3 <$Revision: 1913912 $>
Copyright 1996 Adam Twiss, Zeus Technology Ltd, http://www.zeustech.net/
Licensed to The Apache Software Foundation, http://www.apache.org/

Benchmarking localhost (be patient)
Completed 300 requests
Completed 600 requests
Completed 900 requests
Completed 1200 requests
Completed 1500 requests
Completed 1800 requests
Completed 2100 requests
Completed 2400 requests
Completed 2700 requests
Completed 3000 requests
Finished 3000 requests


Server Software:
Server Hostname:        localhost
Server Port:            8081

Document Path:          /?i=raw.lep
Document Length:        2612717 bytes

Concurrency Level:      100
Time taken for tests:   237.221 seconds
Complete requests:      3000
Failed requests:        0
Total transferred:      7838397000 bytes
HTML transferred:       7838151000 bytes
Requests per second:    12.65 [#/sec] (mean)
Time per request:       7907.354 [ms] (mean)
Time per request:       79.074 [ms] (mean, across all concurrent requests)
Transfer rate:          32268.21 [Kbytes/sec] received

Connection Times (ms)
              min  mean[+/-sd] median   max
Connect:        0    0   0.6      0      12
Processing:   204 7865 1726.4   7860   14071
Waiting:      189 7847 1723.3   7845   14059
Total:        205 7865 1726.4   7860   14071

Percentage of the requests served within a certain time (ms)
  50%   7860
  66%   8441
  75%   8897
  80%   9141
  90%   9983
  95%  10770
  98%  11727
  99%  12178
 100%  14071 (longest request)
```
