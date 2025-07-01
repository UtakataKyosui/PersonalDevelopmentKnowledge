---
aliases:
---
`Loco`は内部的に`Axum`に依存している。
つまり、`Axum`に依存している部分に対して、[`axum-protobuf`](https://github.com/Stefanuk12/axum-protobuf)を利用してあげればいい。
もしくは[`axum_extra`](https://docs.rs/axum-extra/latest/axum_extra/protobuf/struct.Protobuf.html)を使うという方法がある。

