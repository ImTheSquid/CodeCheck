fn loss(truth: &[i32; 4], predict: &[i32; 4]) -> f64 {
    // Assign values
    let t_l1 = truth[0];
    let t_l2 = truth[1];
    let t_r1= truth[2];
    let t_r2= truth[3];

    let p_l1 = predict[0];
    let p_l2 = predict[1];
    let p_r1 = predict[2];
    let p_r2 = predict[3];

    // Calculate each loss
    let l_giou = giou(t_l1, t_l2, p_l1, p_l2); 
    let r_giou = giou(t_r1, t_r2, p_r1, p_r2);

    let loss = l_giou + r_giou; // this is where we would apply function g

    return loss;
}

fn giou(t1: i32, t2: i32, p1: i32, p2: i32) -> f64 {
    // println!("t1: {}, t2: {}, p1: {}, p2: {}", t1, t2, p1, p2);
    let intersection = (t2.min(p2) - t1.max(p1)).max(0);
    // println!("Intersection: {}", intersection);
    let area = p2 - p1 + t2 - t1 - intersection;
    // println!("Area: {}", area);
    let mut entire = 2.max(p2) - t1.min(p1);
    // println!("Entire: {}", entire);

    entire = entire / area; // this is where we would apply function f(C)

    let overlap: f64 = ((intersection as f64 / area as f64) - (entire as f64)).into();
    // println!("Overlap: {}", overlap);
    return overlap;
}

fn main() {
    let truth = [3, 9, 3, 9];
    let predict = [3, 20, 3, 9];
    let loss = loss(&truth, &predict);
    println!("Loss: {}", loss);
}