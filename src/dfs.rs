use std::fs;

fn dfs(graph: &Vec<Vec<i32>>, color: &mut Vec<u8>, start: usize) {
    color[start] = 1;
    for &u in &graph[start] {
       if (color[u as usize] == 0) {
           dfs(graph, color, u as usize);
       }
    }
}

pub fn main() {
    let mut input: String = fs::read_to_string("test/graph.txt").expect("File read error");
    let input_data: Vec<i32> = input.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let n: usize = input_data[0] as usize;
    let m: usize = input_data[1] as usize;
    let mut graph: Vec<Vec<i32>> = vec![Vec::new(); n];
    let mut color: Vec<u8> = vec![0; n];
    let mut i: usize = 2;
    while (i < input_data.len() - 1) {
        let a: i32 = input_data[i] - 1;
        let b: i32 = input_data[i + 1] - 1;
        graph[a as usize].push(b);
        graph[b as usize].push(a);
        i += 2;
    }
    println!("before: {:?}", color);
    dfs(&graph, &mut color, 0);
    println!("after: {:?}", color);

}
