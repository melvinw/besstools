# Some Tools for BESS

There are two crates here:
* `libbess` a rust version of the BESS gRPC client.
* `tools` a few random tools for interacting with BESS, including a version of
bessctl built with clap.

This currently requires a build of BESS with introspection support, like this
branch: https://github.com/melvinw/bess/tree/dyn_pb.
