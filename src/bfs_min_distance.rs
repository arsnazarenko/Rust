use std::fs;
use std::collections::VecDeque;

fn bfs(v: i32, graph: &Vec<Vec<i32>>, color: &mut Vec<u8>,
       distances: &mut Vec<i32>, parents: &mut Vec<i32>, n: usize) {
    let mut q: VecDeque<i32> = VecDeque::new();
    q.push_back(v);
    color[v as usize] = 1;
    while (!q.is_empty()) {
        //  вершина из которой смотрим
        let vert: i32 = q.pop_front().unwrap();
        //  пробегаемся по всем смежным к закрашенной вершине
        for u in &graph[vert as usize] {
            //  если вершина еще не посещалась, закрашиваем ее
            //  добавляем в очереди для проверки всех смежных вершин
            //  обновляем длину пути до текущей вершины
            //  сохраняем предка - вершину, в которую мы пришли из нее
            //  (tckb vs ghb)
            if (color[*u as usize] == 0) {
                q.push_back(*u);
                color[*u as usize] = 1;
                distances[*u as usize] = distances[vert as usize] + 1;
                parents[*u as usize] = vert;
            }
        }
    }
}

pub fn main() {
    let mut input: String = fs::read_to_string("test/graph.txt").expect("File read error");
    let input_data: Vec<i32> = input.split_ascii_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    //  количество вершин
    let n: usize = input_data[0] as usize;
    //  количество ребер
    let m: usize = input_data[1] as usize;
    //  представление графа - списки смежности для каждой вершины
    let mut graph: Vec<Vec<i32>> = vec![Vec::new(); n];
    let mut i: usize = 2;
    while (i < input_data.len() - 1) {
        let a: i32 = input_data[i] - 1;
        let b: i32 = input_data[i + 1] - 1;
        graph[a as usize].push(b);
        graph[b as usize].push(a);
        i += 2;
    }
    //  массив длин путей до каждой вершины
    let mut distances: Vec<i32> = vec![0; n];
    //  для каждой вершины сохраним предка - вершину, из которой мы пришли в нее для восстановления пути
    let mut parents: Vec<i32> = vec![0; n];
    //  в начальную вершину мы не пришли не из какой, пометим это -1
    let start: i32 = 0;
    parents[start as usize] = -1;
    //  цвета вершин
    let mut color: Vec<u8> = vec![0; n];
    bfs(start, &graph, &mut color, &mut distances, &mut parents, n);
    let mut k: i32 = 0;
    let mut way: Vec<i32> = Vec::new();
    println!("{:?}", parents);
    for i in distances {
        println!("Вершина: {}, длина кратчайшего пути: {}", k + 1, i);
        let mut cur: i32 = k;
        while (cur != -1) {
            way.push(cur + 1);
            cur = parents[cur as usize];
        }
        way.reverse();
        way.iter().for_each(|x| print!("{} ", x));
        print!("\n");
        way.clear();
        k += 1;
    }

}