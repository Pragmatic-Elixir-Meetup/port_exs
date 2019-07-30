
###

Elixir 通过 Port 调用 Rust 进程

Steven Gu - Rails
steven.gu@letote.cn

###

NIF 和 Port

1. NIF - https://github.com/rusterlium/rustler
2. Port - http://erlang.org/doc/man/erlang.html#open_port-2

###

Letote 应用

1. RFID 设备标识衣服或者首饰
2. 通过 RFID 识别码跟踪状态
3. 使用 RFID 设备提供商的 SDK

###

选择 Port 的原因

1. Letote 的相关服务构建在 Elixir/Phoenix 基础上
2. RFID 设备提供商的 SDK 是 C lib，内部包含线程（pthread）
3. RFID 设备提供商的 SDK 使用 Callback Function 通知识别码

###

选择 Rust 的原因

1. 实验尝试
2. C 的结构化封装有难度（glibc，ffmpeg），C++ 成本高
3. Rust 友好的编译期内存检测
4. Cargo 工具链针对多平台 Targets 的支持，省去了 C Makefile 的复杂度

###

Rust 相关库

1. rust-bindgen - https://github.com/rust-lang/rust-bindgen（C binding）
2. libc - https://github.com/rust-lang/libc (pipe, select, epoll 等)
3. nix - https://github.com/nix-rust/nix (u8, i8, c_char, errno 等)

###

Simple Example

```
mix new simple_ex
cd simple_ex
cargo init
```

###

Mix Task

```
cargo build --bin simple_ex --release --target-dir \
    _build/dev/lib/simple_ex/priv
```

```
defmodule Mix.Tasks.Compile.Cargo do
  use Mix.Task

  defp build do
    System.cmd("cargo", build_cmd_args, [
      stderr_to_stdout: true,
      into: IO.stream(:stdio, :line)
    ])
  end
end
```

###

Elixir Part（1）

```
defmodule SimpleEx do
  use GenServer

  def init(nil) do
    executable = :code.priv_dir(:simple_ex) ++ '/release/simple_ex'

    port = Port.open({:spawn_executable, executable}, [
      {:packet, 2},
      :binary,
      :exit_status,
      :use_stdio
      ])
  end
end
```

###

Elixir Part（2）

```
  def handle_info({_port, {:data, << data :: binary >>}}, state) do
    code = :erlang.binary_to_term(data)

    IO.puts "elixir_SimpleEx: Receives - #{code}"

    {:noreply, state}
  end
```

###

Rust Part

```
let mut sender = io::stdout();

sender.
  write_u16::<BigEndian>(encoded_data.len() as u16).
  expect("failed to write data size");

sender.write_all(&encoded_data).expect("failed to write data");

sender.flush().expect("failed to flush stdout");
```

Finishes Simple Example

###

Other complicated ones

`https://github.com/silathdiir/port_exs/fancy_ex`

1. Elixir 通过 Port 调用 C 进程
2. Elixir 通过 Port 调用 Python 进程

