use polars::{
    df,
    frame::DataFrame,
    prelude::{NamedFrom, PolarsResult},
    series::Series,
};

fn polars_test() {
    //let df_input = df!("Fruit" => &["Apple", "Apple", "Pear"],"Color" => &["Red", "Yellow", "Green"]).unwrap();
    let s1 = Series::new("Fruit", &["Apple", "Apple", "Pear"]);
    let s2 = Series::new("Color", &["Red", "Yellow", "Green"]);

    let df_input = DataFrame::new(vec![s1, s2]).unwrap();
    println!("{}", df_input);
    //let df = DataFrame::default();
}
