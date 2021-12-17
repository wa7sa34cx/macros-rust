macro_rules! bar {
    () => {
        println!("Hello from macro");
    };
}

pub(crate) use bar;
