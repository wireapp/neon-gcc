#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult};
use neon::js::JsString;

fn hello(call: Call) -> JsResult<JsString> {
    Ok(JsString::new(call.scope, "Hello gcc").unwrap())
}

register_module!(m, {
    m.export("hello", hello)
});
