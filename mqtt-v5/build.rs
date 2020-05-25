
fn main() {
  let kremlin_home = std::env::var("KREMLIN_HOME")
  .expect("$KREMLIN_HOME is not defined");

  cc::Build::new()
      .warnings(true)
      .file("../../mqtt-packet-parser/out/Main.c")
      .file("../../mqtt-packet-parser/utils.c")
      .include("../../mqtt-packet-parser/out")
      .include(kremlin_home + "/include")
      .compiler("/usr/bin/llvm-gcc")
      .compile("libmqtt.a");
}