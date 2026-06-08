use std::time::Instant;

use anyhow::{Context, Result};
use wasmtime::component::Component;
use wasmtime::component::{Linker, ResourceTable};
use wasmtime::{Engine, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiView};

use crate::sandbox::SandboxError;
use crate::types::ToolConfig;

struct SandboxWasi {
    ctx: WasiCtx,
    table: ResourceTable,
    output: Vec<u8>,
}

impl WasiView for SandboxWasi {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }

    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}

pub fn execute_wasm(
    engine: &Engine,
    wasm_bytes: &[u8],
    func_name: &str,
    args: &str,
    config: &ToolConfig,
) -> Result<String, SandboxError> {
    let start = Instant::now();

    let component = Component::new(engine, wasm_bytes)?;

    let mut wasi_ctx = SandboxWasi {
        ctx: WasiCtxBuilder::new()
            .inherit_stdout()
            .inherit_stderr()
            .inherit_stdin()
            .args(&["wasm-module", func_name, args])
            .build(),
        table: ResourceTable::default(),
        output: Vec::new(),
    };

    let mut store = Store::new(engine, wasi_ctx);
    store.set_epoch_deadline(config.timeout);
    store.set_fuel(config.memory_limit as u64)?;

    let mut linker = Linker::new(engine);
    wasmtime_wasi::add_to_linker_sync(&mut linker).context("Failed to add WASI to linker")?;

    let instance = linker
        .instantiate(&mut store, &component)
        .map_err(|e| SandboxError::Other(format!("Instantiation failed: {}", e)))?;

    let func = instance
        .get_func(&mut store, func_name)
        .ok_or_else(|| SandboxError::Other(format!("Function '{}' not found", func_name)))?;

    let params: Vec<wasmtime::Val> = vec![wasmtime::Val::I64(args.len() as i64)];
    let mut results = vec![wasmtime::Val::I64(0)];

    func.call(&mut store, params.as_slice(), &mut results)
        .map_err(|e| {
            let elapsed = start.elapsed().as_millis() as u64;
            if elapsed >= config.timeout {
                SandboxError::Timeout {
                    timeout: config.timeout,
                }
            } else {
                SandboxError::Other(format!("Execution failed: {}", e))
            }
        })?;

    let result = match results.get(0) {
        Some(wasmtime::Val::I64(val)) => val.to_string(),
        Some(wasmtime::Val::I32(val)) => val.to_string(),
        Some(wasmtime::Val::ExternRef(Some(r))) => format!("{:?}", r.data(&store)),
        _ => "0".to_string(),
    };

    Ok(result)
}

pub fn execute_with_memory_limit(
    engine: &Engine,
    wasm_bytes: &[u8],
    func_name: &str,
    args: &str,
    config: &ToolConfig,
    memory_limit: usize,
) -> Result<String, SandboxError> {
    let mut limited_config = config.clone();
    limited_config.memory_limit = memory_limit;
    execute_wasm(engine, wasm_bytes, func_name, args, &limited_config)
}
