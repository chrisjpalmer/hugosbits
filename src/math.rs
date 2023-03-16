fn cartesian_product<T: Clone>(arrays: &[&[T]]) -> Vec<Vec<T>> {
    let mut product: Vec<Vec<T>> = vec![vec![]];

    for array in arrays {
        let mut new_product = Vec::new();

        for item in array.iter() {
            for product_item in product.iter() {
                let mut new_item = product_item.clone();
                new_item.push(item.clone());
                new_product.push(new_item);
            }
        }

        product = new_product;
    }

    product
}

#[test]
fn test_cartesian_product() {
    let a = [1, 2];
    let b = [3, 4];
    let c = [5, 6];

    let result = cartesian_product(&[&a, &b, &c]);

    assert_eq!(result, vec![
        vec![1, 3, 5],
        vec![1, 3, 6],
        vec![1, 4, 5],
        vec![1, 4, 6],
        vec![2, 3, 5],
        vec![2, 3, 6],
        vec![2, 4, 5],
        vec![2, 4, 6]
    ]);
}
