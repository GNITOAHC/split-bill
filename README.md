# split-bill

## Execution

```
cargo run
```

then visit [localhost:8000](http://localhost:8000).

### Query Parameters

> | key | required | data type | description                      |
> | --- | -------- | --------- | -------------------------------- |
> | a:b | true     | float     | Indicates that a owe b x dollars |

> e.g. http://localhost:8000/calc-balance?a:b=123&b:a=32&a:b=43&a:c=1234  
> a owe b 123 dollars  
> b owe a 32 dollars  
> a owe b 43 dollars  
> a owe c 1234 dollars  
