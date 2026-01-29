use macros::Builder;

#[derive(Builder, Debug)]
struct User<T> {
    name: String,

    #[builder(default = 18)]
    age: u32,

    #[builder(skip)]
    id: u64,

    data: T,
}

fn main() {
    let u = User::builder()
        .name("Alice".into())
        .data(123)
        .build()
        .unwrap();

    assert_eq!("Alice", u.name);
    assert_eq!(18, u.age);
    assert_eq!(0, u.id);
    assert_eq!(123, u.data);
}
