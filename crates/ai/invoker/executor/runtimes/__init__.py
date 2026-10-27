"""One module per runtime. Each has START_FIELDS (the `start` fields it
takes besides runtime and heartbeat_timeout_secs) and a Runtime class:
Runtime(start, children) loads the model, versions() names what it runs on,
and handle(job) does one request and returns the fields of `done`. Living in
this package keeps their names clear of the libraries they use: a top-level
qwen_tts.py would shadow the qwen_tts package."""
