pub fn sum_array25(a: &[f64; 26], b: &[f64; 26]) -> [f64; 26] {
    let mut collection: [f64; 26] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

    (1..26).for_each(|index| {
        collection[index] = a[index] + b[index];
    });

    collection[0] = a[0];

    collection
}

pub fn sub_array25(a: &[f64; 26], b: &[f64; 26]) -> [f64; 26] {
    let mut collection: [f64; 26] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

    (1..26).for_each(|index| {
        collection[index] = a[index] - b[index];
    });

    collection[0] = a[0];

    collection
}

pub fn sum_elm_array(array: &[f64; 26]) -> f64 {
    let mut sum: f64 = 0.0;

    (1..26).for_each(|index| {
        sum += array[index];
    });

    sum
}


pub fn sqr_array25(a: &[f64; 26]) -> [f64; 26] {
    let mut collection: [f64; 26] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

    (1..26).for_each(|index| {
        collection[index] = a[index] * a[index];
    });

    collection[0] = a[0];

    collection
}


pub fn sqr_root_array25(a: &[f64; 26]) -> [f64; 26] {
    let mut collection: [f64; 26] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

    (1..26).for_each(|index| {
        collection[index] = a[index].sqrt();
    });

    collection[0] = a[0];

    collection
}