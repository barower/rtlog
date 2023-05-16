This little library is based on Chris Apple's talk during ADCx 2023 in San Francisco
He also published his library here: https://github.com/cjappl/rtlog-cpp

# Requirements for real-time logging library

On real-time thread:
- No system calls (how to check that?)
- No allocations (use format_args! or enums that implement fmt::Display instead of Strings, use mpsc::sync_channel)
- No mutexes (solved by mpsc::sync_channel)

