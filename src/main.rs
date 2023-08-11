use v8 as v8;
use v8::Global as Global;

fn main() {
    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);
    let context = v8::Context::new(scope);
//    let global_context = Global::new(scope, context);
//    let context = v8::Local::new(scope, &global_context);

    let scope = &mut v8::ContextScope::new(scope, context);
    
    let code = v8::String::new(scope, "const bar = 'bar'; const foo = { foo: 'foo' }; foo").unwrap();
    println!("javascript code: {}", code.to_rust_string_lossy(scope));

    let script = v8::Script::compile(scope, code, None).unwrap();

    let result = script.run(scope).unwrap();
    let result2 = result.to_string(scope).unwrap();
    println!("result: {:?}", result2.to_rust_string_lossy(scope));


    // create new context from the same scope, so references are kept
    let context = v8::Context::new(scope);
    let global_context = Global::new(scope, context);
    let context = v8::Local::new(scope, &global_context);
    let scope = &mut v8::ContextScope::new(scope, context);

    // add result from previous script to the new scope's global object
    let foo = v8::String::new(scope, "foo").unwrap();
    let foo_val = result;
    let global = context.global(scope);
    global.set(scope, foo.into(), foo_val.into());

    let scope2 = &mut v8::HandleScope::new(scope);

    // return foo.foo
    let code = v8::String::new(scope2, "const bar='bar'; foo.foo").unwrap();
    println!("javascript code: {}", code.to_rust_string_lossy(scope2));

    let script = v8::Script::compile(scope2, code, None).unwrap();

    let result = script.run(scope2).unwrap();
    let result = result.to_string(scope2).unwrap();
    println!("result: {:?}", result.to_rust_string_lossy(scope2));

    // how can we prevent global.bar from being set here?
    // where global.foo remains set, and shared (will be immutable later)
    // cannot create a new scope using the same global scope, by reference
    let code = v8::String::new(scope2, "bar").unwrap();
    println!("javascript code: {}", code.to_rust_string_lossy(scope2));

    let script = v8::Script::compile(scope2, code, None).unwrap();

    let result = script.run(scope2).unwrap();
    let result = result.to_string(scope2).unwrap();
    println!("result: {:?}", result.to_rust_string_lossy(scope2));

}
