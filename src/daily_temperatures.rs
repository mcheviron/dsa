// pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
//     let mut res = vec![0; temperatures.len()];
//     let mut stack = vec![];

//     for (i, &t) in temperatures.iter().enumerate() {
//         while let Some(&j) = stack.last() {
//             if t <= temperatures[j] {
//                 break;
//             }
//             res[j] = (i - j) as i32;
//             stack.pop();
//         }
//         stack.push(i);
//     }

//     res
// }
pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; temperatures.len()];
    let mut stack = vec![];
    for (i, &t) in temperatures.iter().enumerate() {
        while let Some(&j) = stack.last() {
            if t <= temperatures[j] {
                break;
            }
            res[j] = (i - j) as i32;
            stack.pop();
        }
        stack.push(i);
    }
    res
}
