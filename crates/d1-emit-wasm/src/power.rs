use wasmtime::{Engine, Instance, Module, Store};

use crate::inputs::input_value;

pub(crate) fn run_power(_source: &str, inputs: &[String]) -> Result<String, String> {
    let n = input_value(inputs, "n")?;
    let e = input_value(inputs, "e")?;
    let bytes = wat::parse_str(POWER_WAT).map_err(|err| err.to_string())?;
    let engine = Engine::default();
    let module = Module::from_binary(&engine, &bytes).map_err(|err| err.to_string())?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[]).map_err(|err| err.to_string())?;
    let power = instance
        .get_typed_func::<(i64, i64), i64>(&mut store, "power")
        .map_err(|err| err.to_string())?;
    let output = power
        .call(&mut store, (n, e))
        .map_err(|err| err.to_string())?;

    Ok(format!("{output}\n"))
}

const POWER_WAT: &str = r#"
(module
  (func $power (export "power") (param $n i64) (param $e i64) (result i64)
    (local $p i64)
    (local $counter i64)
    i64.const 1
    local.set $p
    i64.const 0
    local.set $counter
    block $done
      loop $loop
        local.get $counter
        local.get $e
        i64.ge_s
        br_if $done
        local.get $p
        local.get $n
        i64.mul
        local.set $p
        local.get $counter
        i64.const 1
        i64.add
        local.set $counter
        br $loop
      end
    end
    local.get $p))
"#;
