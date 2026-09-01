# A theory teest (part 1)

The goal is to build a fully functional small backend project based on all of the already accumulated knowledge to find weakpoints in understanding of the design architecture. The task was created by GPT, but the code is fully self-written. 

## Requirements

Create a Rust program in which:

The client submits commands to a server.

The server dispatches work to a fixed number of worker threads.

Multiple commands may be processed concurrently.

Workers operate on shared application state.

The state must remain internally consistent.

Every command gets a response.

The client can request shutdown.

Shutdown must be graceful:
no new work should be accepted after shutdown begins;
already accepted work should be handled according to a clearly defined policy;
worker threads must terminate;
main must not simply abandon them.

Errors must be represented rather than handled with unwrap() throughout the actual application logic.

Do not solve concurrency by spawning one thread per command.
 
