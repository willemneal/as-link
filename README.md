## AS Link

This is an example of using wasmtime to link two AssemblyScript Wasm binaries together at load time.

### Requirements

- Node
- Cargo
- Optional: yarn

## AS files
a:
```ts
@external("b", "answerToLife")
declare function answerToLife(): i32;


if (answerToLife() != 42) {
  unreachable();
}

export function run(): i32 {
  return answerToLife();
}
```
b:
```ts
export function answerToLife(): i32 {
  return 42;
}
```

Compile respectively to:
`a.wat`:
```wat
(module
 (type $none_=>_i32 (func (result i32)))
 (type $none_=>_none (func))
 (import "b" "answerToLife" (func $packages/a/assembly/index/answerToLife (result i32)))
 (memory $0 0)
 (export "run" (func $packages/a/assembly/index/run))
 (start $~start)
 (func $packages/a/assembly/index/run (result i32)
  call $packages/a/assembly/index/answerToLife
 )
 (func $~start
  call $packages/a/assembly/index/answerToLife
  i32.const 42
  i32.ne
  if
   unreachable
  end
 )
)
```
`b.wat`:
```wat
(module
 (type $none_=>_i32 (func (result i32)))
 (memory $0 0)
 (export "answerToLife" (func $packages/b/assembly/index/answerToLife))
 (func $packages/b/assembly/index/answerToLife (result i32)
  i32.const 42
 )
)
```

## Usage

```
yarn build
yarn test
```

Should print `42`.