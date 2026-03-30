COMPRESSION BENCHMARK REPORT
Generated: Fri Mar 27 09:59:58 CET 2026
Tools:     tar (gzip/bzip2/xz) · zip · rat (fast/default/best)
══════════════════════════════════════════════════════════════════════

▶ ROUND 1 — RANDOM DATA (high entropy, near-incompressible)

Size            tar+gzip                  tar+bzip2                 tar+xz                    zip                       rat(fast)                 rat(default)              rat(best)               
──────────────────────────────────────────────────────────────────────
10.00 KB        10.20 KB / 8ms (+2.0%)    10.59 KB / 7ms (+5.9%)    10.30 KB / 26ms (+3.0%)   10.22 KB / 13ms (+2.2%)   10.73 KB / 6ms (+7.3%)    10.73 KB / 3ms (+7.3%)    10.73 KB / 3ms (+7.3%)  
25.00 KB        25.18 KB / 3ms (+.7%)     25.58 KB / 6ms (+2.3%)    25.50 KB / 25ms (+2.0%)   25.22 KB / 2ms (+.9%)     25.72 KB / 5ms (+2.8%)    25.72 KB / 4ms (+2.8%)    25.72 KB / 4ms (+2.8%)  
50.00 KB        50.24 KB / 4ms (+.4%)     50.69 KB / 8ms (+1.3%)    50.88 KB / 24ms (+1.7%)   50.23 KB / 3ms (+.4%)     50.80 KB / 7ms (+1.6%)    50.80 KB / 6ms (+1.6%)    50.80 KB / 6ms (+1.6%)  
200.00 KB       200.26 KB / 9ms (+.1%)    201.43 KB / 20ms (+.7%)   200.90 KB / 60ms (+.4%)   200.26 KB / 6ms (+.1%)    202.32 KB / 18ms (+1.1%)  201.52 KB / 17ms (+.7%)   201.52 KB / 18ms (+.7%) 
500.00 KB       500.31 KB / 16ms (+0%)    502.82 KB / 44ms (+.5%)   500.98 KB / 112ms (+.1%)  500.30 KB / 12ms (+0%)    504.64 KB / 43ms (+.9%)   502.98 KB / 41ms (+.5%)   502.98 KB / 40ms (+.5%) 
2.00 MB         2.00 MB / 53ms (+0%)      2.00 MB / 166ms (+.4%)    2.00 MB / 445ms (+0%)     2.00 MB / 47ms (+0%)      2.01 MB / 147ms (+.8%)    2.01 MB / 155ms (+.5%)    2.00 MB / 164ms (+.4%)  
10.00 MB        10.00 MB / 255ms (+0%)    10.04 MB / 819ms (+.4%)   10.00 MB / 3121ms (+0%)   10.00 MB / 223ms (+0%)    10.08 MB / 786ms (+.8%)   10.05 MB / 780ms (+.5%)   10.04 MB / 813ms (+.4%) 
512.00 MB       512.08 MB / 12925ms (+0%)  514.26 MB / 42825ms (+.4%)  512.02 MB / 199610ms (+0%)  512.08 MB / 11806ms (+0%)  516.11 MB / 41808ms (+.8%)  514.66 MB / 41658ms (+.5%)  514.26 MB / 43739ms (+.4%)
1.00 GB         1.00 GB / 26573ms (+0%)   1.00 GB / 86832ms (+.4%)  1.00 GB / 412251ms (+0%)  1.00 GB / 23396ms (+0%)   1.00 GB / 83095ms (+.8%)  1.00 GB / 83365ms (+.5%)  1.00 GB / 88578ms (+.4%)
3.00 GB         3.00 GB / 81145ms (+0%)   3.01 GB / 261473ms (+.4%)  3.00 GB / 1251472ms (+0%)  3.00 GB / 71986ms (+0%)   3.02 GB / 248039ms (+.8%)  3.01 GB / 251063ms (+.5%)  3.01 GB / 267982ms (+.4%)

Format: compressed_size / time_ms (size vs original)
──────────────────────────────────────────────────────────────────────

SUMMARY — winner per file size
──────────────────────────────────────────────────────────────────────
File size       Smallest            Size                    Fastest             Time                  
10.00 KB        tar_gz              10.20 KB                rat_default         3ms                   
25.00 KB        tar_gz              25.18 KB                zip                 2ms                   
50.00 KB        zip                 50.23 KB                zip                 3ms                   
200.00 KB       zip                 200.26 KB               zip                 6ms                   
500.00 KB       zip                 500.30 KB               zip                 12ms                  
2.00 MB         tar_gz              2.00 MB                 zip                 47ms                  
10.00 MB        tar_xz              10.00 MB                zip                 223ms                 
512.00 MB       tar_xz              512.02 MB               zip                 11806ms               
1.00 GB         ?                   953.67 MB               zip                 23396ms               
3.00 GB         ?                   953.67 MB               zip                 71986ms               
──────────────────────────────────────────────────────────────────────

▶ ROUND 2 — MIXED DATA (50% repeated pattern + 50% random, interleaved 4KB chunks)

Size            tar+gzip                  tar+bzip2                 tar+xz                    zip                       rat(fast)                 rat(default)              rat(best)               
──────────────────────────────────────────────────────────────────────
10.00 KB        6.28 KB / 7ms (-37.1%)    6.77 KB / 8ms (-32.2%)    6.30 KB / 17ms (-36.9%)   6.34 KB / 3ms (-36.5%)    6.91 KB / 4ms (-30.8%)    6.91 KB / 4ms (-30.8%)    6.91 KB / 4ms (-30.8%)  
25.00 KB        13.32 KB / 4ms (-46.7%)   13.79 KB / 8ms (-44.8%)   13.41 KB / 22ms (-46.3%)  13.41 KB / 2ms (-46.3%)   13.93 KB / 8ms (-44.2%)   13.93 KB / 6ms (-44.2%)   13.93 KB / 7ms (-44.2%) 
50.00 KB        26.44 KB / 4ms (-47.1%)   26.85 KB / 12ms (-46.2%)  26.61 KB / 22ms (-46.7%)  26.51 KB / 3ms (-46.9%)   26.99 KB / 12ms (-46.0%)  26.99 KB / 10ms (-46.0%)  26.99 KB / 10ms (-46.0%)
200.00 KB       101.22 KB / 6ms (-49.3%)  101.29 KB / 35ms (-49.3%)  101.76 KB / 37ms (-49.1%)  101.29 KB / 5ms (-49.3%)  102.30 KB / 36ms (-48.8%)  101.43 KB / 37ms (-49.2%)  101.43 KB / 37ms (-49.2%)
500.00 KB       254.65 KB / 11ms (-49.0%)  254.16 KB / 90ms (-49.1%)  255.98 KB / 71ms (-48.8%)  254.72 KB / 9ms (-49.0%)  257.19 KB / 83ms (-48.5%)  254.28 KB / 93ms (-49.1%)  254.28 KB / 95ms (-49.1%)
2.00 MB         1.01 MB / 32ms (-49.4%)   1.00 MB / 381ms (-49.6%)  1.01 MB / 240ms (-49.2%)  1.01 MB / 30ms (-49.4%)   1.01 MB / 320ms (-49.1%)  1.00 MB / 366ms (-49.5%)  1.00 MB / 405ms (-49.6%)
10.00 MB        5.04 MB / 154ms (-49.5%)  5.03 MB / 1899ms (-49.6%)  5.07 MB / 1464ms (-49.2%)  5.04 MB / 135ms (-49.5%)  5.08 MB / 1579ms (-49.1%)  5.03 MB / 1937ms (-49.6%)  5.03 MB / 1994ms (-49.6%)
512.00 MB       258.52 MB / 7266ms (-49.5%)  257.82 MB / 100013ms (-49.6%)  259.79 MB / 85810ms (-49.2%)  258.52 MB / 6791ms (-49.5%)  260.47 MB / 80694ms (-49.1%)  257.96 MB / 96958ms (-49.6%)  257.82 MB / 102659ms (-49.6%)
1.00 GB         517.05 MB / 14629ms (-49.5%)  515.64 MB / 194896ms (-49.6%)  519.59 MB / 172181ms (-49.2%)  517.05 MB / 13809ms (-49.5%)  520.94 MB / 161609ms (-49.1%)  515.92 MB / 190671ms (-49.6%)  515.64 MB / 206881ms (-49.6%)
3.00 GB         1.51 GB / 43918ms (-49.5%)  1.51 GB / 583571ms (-49.6%)  1.52 GB / 534015ms (-49.2%)  1.51 GB / 43258ms (-49.5%)  1.52 GB / 506509ms (-49.1%)  1.51 GB / 614042ms (-49.6%)  1.51 GB / 651862ms (-49.6%)

Format: compressed_size / time_ms (size vs original)
──────────────────────────────────────────────────────────────────────

SUMMARY — winner per file size
──────────────────────────────────────────────────────────────────────
File size       Smallest            Size                    Fastest             Time                  
10.00 KB        tar_gz              6.28 KB                 zip                 3ms                   
25.00 KB        tar_gz              13.32 KB                zip                 2ms                   
50.00 KB        tar_gz              26.44 KB                zip                 3ms                   
200.00 KB       tar_gz              101.22 KB               zip                 5ms                   
500.00 KB       tar_bz2             254.16 KB               zip                 9ms                   
2.00 MB         tar_bz2             1.00 MB                 zip                 30ms                  
10.00 MB        tar_bz2             5.03 MB                 zip                 135ms                 
512.00 MB       tar_bz2             257.82 MB               zip                 6791ms                
1.00 GB         rat_best            515.64 MB               zip                 13809ms               
3.00 GB         ?                   953.67 MB               zip                 43258ms               
──────────────────────────────────────────────────────────────────────

▶ ROUND 3 — APPEND TEST (zip -u vs rat, adding 3 files to existing archive)
  tar skipped: compressed tar does not support efficient appending.

Base size       zip (-u)                        rat(fast)                       rat(default)                    rat(best)                     
──────────────────────────────────────────────────────────────────────
10.00 KB        40.86 KB (+30.63 KB / 5ms)      42.91 KB (+32.17 KB / 12ms)     42.91 KB (+32.17 KB / 11ms)     42.91 KB (+32.17 KB / 10ms)   
25.00 KB        100.86 KB (+75.63 KB / 5ms)     102.84 KB (+77.12 KB / 13ms)    102.84 KB (+77.12 KB / 14ms)    102.84 KB (+77.12 KB / 13ms)  
50.00 KB        200.90 KB (+150.66 KB / 8ms)    203.20 KB (+152.39 KB / 21ms)   203.20 KB (+152.39 KB / 19ms)   203.20 KB (+152.39 KB / 18ms) 
200.00 KB       801.00 KB (+600.74 KB / 19ms)   809.24 KB (+606.91 KB / 58ms)   806.15 KB (+604.63 KB / 54ms)   806.15 KB (+604.63 KB / 53ms) 
500.00 KB       1.95 MB (+1.46 MB / 40ms)       1.97 MB (+1.47 MB / 132ms)      1.96 MB (+1.47 MB / 131ms)      1.96 MB (+1.47 MB / 136ms)    
2.00 MB         8.00 MB (+6.00 MB / 158ms)      8.06 MB (+6.04 MB / 505ms)      8.04 MB (+6.03 MB / 512ms)      8.03 MB (+6.02 MB / 562ms)    
10.00 MB        40.00 MB (+30.00 MB / 743ms)    40.32 MB (+30.24 MB / 2560ms)   40.21 MB (+30.15 MB / 2565ms)   40.18 MB (+30.13 MB / 2822ms) 
512.00 MB       2.00 GB (+1.50 GB / 38337ms)    2.01 GB (+1.51 GB / 129763ms)   2.01 GB (+1.50 GB / 125356ms)   2.00 GB (+1.50 GB / 126712ms) 
1.00 GB         4.00 GB (+3.00 GB / 74656ms)    4.03 GB (+3.02 GB / 243636ms)   4.02 GB (+3.01 GB / 250786ms)   4.01 GB (+3.01 GB / 263170ms) 

Format: total_archive_size (+bytes_added_by_append / time_ms)
Append test: 3 files of same size added to existing archive.
──────────────────────────────────────────────────────────────────────

NOTES:
  Mixed files: 4KB chunks of ASCII pattern alternating with 4KB of urandom.
  Append test: base archive created first (not timed), then 3 extra files
               of the same size are appended and timed together.
  Large files (500MB+) use /dev/urandom, expect near-zero compression.
══════════════════════════════════════════════════════════════════════
