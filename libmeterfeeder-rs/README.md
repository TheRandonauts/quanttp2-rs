# Libmeter rust wrapper

Compile
```
$ cargo build
```

Install
```
$ sudo cp ./libmeterfeed.so /usr/lib 
```

Test
```
$ LD_LIBRARY_PATH=. cargo test
```

Test with outputs
```
$ LD_LIBRARY_PATH=. cargo test -- --nocapture
```
