macro_rules! internal {
    () => {
        println!("Hello from macro");
    };
}

pub(crate) use internal;