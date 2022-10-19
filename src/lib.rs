use std::marker::PhantomData;

#[derive(Debug)]
pub struct MyStruct {
    required_int: usize,
    optional_int: Option<usize>,
    required_string: String,
    optional_string: Option<String>,
    multiple_things: Vec<String>
}

impl MyStruct {
    pub fn explain_me(&self) {
        println!("I am a struct with a required int: {}, an optional int: {:?}, a required string: {}, an optional string: {:?}, and multiple things: {:?}", self.required_int, self.optional_int, self.required_string, self.optional_string, self.multiple_things);
    }
}

pub struct HasInt;
pub struct HasStr;

pub struct MyStructBuilder<State = ((), ())> {
    s: MyStruct,
    _assign_once_state: PhantomData<State>
}

impl MyStructBuilder {
    pub fn new(int: usize, string: impl Into<String>) -> MyStructBuilder {
        MyStructBuilder {
            s: MyStruct {
                required_int: int,
                optional_int: None,
                required_string: string.into(),
                optional_string: None,
                multiple_things: Vec::new()
            },
            _assign_once_state: PhantomData
        }
    }
}

impl<IntState, StrState> MyStructBuilder<(IntState, StrState)> {
    pub fn add_thing(mut self, string: impl Into<String>) -> MyStructBuilder<(IntState, StrState)> {
        self.s.multiple_things.push(string.into());
        self
    }

    pub fn build(self) -> MyStruct {
        self.s
    }
}
impl<StrState> MyStructBuilder<((), StrState)> {
    pub fn optional_int(mut self, int: usize) -> MyStructBuilder<(HasInt, StrState)>
    {
        self.s.optional_int = Some(int);
        MyStructBuilder { s: self.s, _assign_once_state: PhantomData }
    }
}
impl<IntState> MyStructBuilder<(IntState, ())> {
    pub fn optional_string(mut self, string: impl Into<String>) -> MyStructBuilder<(IntState, HasStr)> {
        self.s.optional_string = Some(string.into());
        MyStructBuilder { s: self.s, _assign_once_state: PhantomData }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let builder = MyStructBuilder::new(1, "hello");
        let a: MyStructBuilder<(HasInt, _)> = builder.optional_int(5);
        let s = a
            .optional_string("hi")
            .add_thing("whales")
            .add_thing("are")
            .add_thing("cool")
            // .optional_string("oops")
            // .optional_int(6)
            .build();
        println!("{:?}", s);
        s.explain_me();
    }
}
