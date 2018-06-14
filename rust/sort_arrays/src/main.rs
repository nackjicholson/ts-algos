#[derive(Debug)] 
struct Movie {
    name: &'static str,
    year: i32,
}

fn main() {
    let arr: Vec<&str> = vec!["foo", "bar"];
    let mut sorted_arr: Vec<&str> = arr.clone();
    sorted_arr.sort();
    println!("arr: {:?}", arr);
    println!("sorted_arr: {:?}", sorted_arr);

    let mut foo: [i32; 4] = [1, 3, 2, 22];
    foo.sort_by(|a, b| a.cmp(b));
    println!("sorted foo: {:?}", foo);
    // reverse sorting.
    foo.sort_by(|a, b| b.cmp(a));
    println!("reverse sorted foo: {:?}", foo);

    let mut movies: Vec<Movie> = vec![
        Movie {
            name: "The Shawshank Redemption",
            year: 1994,
        },
        Movie {
            name: "The Godfather",
            year: 1972,
        },
        Movie {
            name: "The Godfather: Part II",
            year: 1974,
        },
        Movie {
            name: "The Dark Knight",
            year: 2008,
        },
    ];

    movies.sort_by(|a, b| a.year.cmp(&b.year));
    println!("sorted movies: {:?}", movies);

    let mut foo_strings: [&str; 5] = [
        "Alpha",
        "beta",
        "Gamma",
        "gramma",
        "delta",
    ];
    foo_strings.sort_by(|a, b| {
        a.to_lowercase().cmp(&b.to_lowercase())
    });
    println!("sorted foo_strings: {:?}", foo_strings);
}
