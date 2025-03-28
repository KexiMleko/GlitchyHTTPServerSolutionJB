# GlitchyHTTPServerSolutionJB

This is a client application for downloading binary data from a glitchy HTTP server provided by JetBrains.

## Language

[Rust]

## Dependencies

- Used sha2 for SHA-256 hashing to ensure correctness and efficiency. Implementing SHA-256 manually was avoided to focus on the core logic of handling partial responses.

## Approach

- Data chunks are set to be less than 64KB since the server fails to send complete data for chunks larger than 64KB.
- The client sends GET request with the Range header to fetch missing data chunks.
- Increment range by specified data chunk.
- It repeats the process until a data chunk (Content-length) from response is smaller than chunk given in request
- Verify the SHA-256 hash of downloaded data trough user input of server generated hash.

## How to Test

- Start the provided Python server in a terminal.
- In this repo’s root, build and run the client using cargo run, executable will eventually run and start downloading the data and printing downloaded chunk lengths.
- When the full download is done, paste server generated hash in the terminal to verify the download.
- Status of the downloaded will be printed in the terminal (successful or incomplete).