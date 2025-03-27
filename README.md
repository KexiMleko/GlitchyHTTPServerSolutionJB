# GlitchyHTTPServerSolutionJB
This is a client application for downloading binary data from a glitchy HTTP server provided by JetBrains. 
## Language
[Rust]

## Dependencies
- Used a sha2 for SHA-256 hashing to ensure correctness and efficiency. Implementing SHA-256 manually was avoided to focus on the core logic of handling partial responses.

## Approach
- Data chunk for request range is chosen based on constraints of the buggy server, so it is set to be smaller than 64KB.
- The client sends GET requests with the Range header to fetch missing data chunks.
- It repeats the process until a data chunk (Content-length) from response is smaller than chunk given in request, then verifies the SHA-256 hash.
