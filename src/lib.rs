pub trait SafeBuilder<T: PartialBuilder>
{
    fn build() -> T;
}

pub trait PartialBuilder { }