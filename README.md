# rustapi
simple restapi server


## Example
```rust
use rustapi::start;

fn main() {
    let mut paths = HashMap::new();
    
    paths.insert("/", serde_json::json!({"status": 200, "message": "hello world"}));
    paths.insert("/test", serde_json::json!({"status": 200, "message": "hello from /test page"}));
    
    assert_eq!(paths.len(), 2);
    
    start("127.0.0.1:5000", paths);
}

```
