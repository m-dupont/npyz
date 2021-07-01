
extern crate npy_derive;
extern crate npy;

// examples/structured.npy is generated by this Python code:
//
// import numpy as np
// a = np.array([(1,2.5,4), (2,3.1,5)], dtype=[('a', 'i4'),('b', 'f4'),('c', 'i8')])
// np.save('examples/structured.npy', a)

#[derive(npy::Deserialize, Debug, PartialEq)]
struct Struct {
    a: i32,
    b: f32,
    c: i64,
}

fn main() -> std::io::Result<()> {
    let bytes = std::fs::read("examples/structured.npy")?;

    let reader = npy::NpyReader::<Struct, _>::new(&bytes[..])?;
    let vec = reader.into_vec()?;
    assert_eq!(vec, vec![
        Struct { a: 1, b: 2.5, c: 4 },
        Struct { a: 2, b: 3.1, c: 5 },
    ]);
    Ok(())
}