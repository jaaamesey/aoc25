#[derive(Debug, Clone)]
struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
    // Vec of indexes
    connections: Vec<usize>,
    back_connections: Vec<usize>,
}

pub fn part1() {
    let input_str = include_str!("./test_input.txt");
    let mut graph = input_str
        .lines()
        .map(|line| {
            let mut split = line.split(',').map(|s| s.parse::<usize>().unwrap());
            JunctionBox {
                x: split.next().unwrap(),
                y: split.next().unwrap(),
                z: split.next().unwrap(),
                connections: Vec::new(),
                back_connections: Vec::new(),
            }
        })
        .collect::<Vec<_>>();

    let mut closest_node_distances = graph
        .iter()
        .map(|node| {
            let closest_node = graph
                .iter()
                .filter(|n| node.x != n.x || node.y != n.y || node.z != n.z)
                .reduce(|closest_node, candidate_node| {
                    if distance_squared_between_nodes(node, candidate_node)
                        < distance_squared_between_nodes(node, closest_node)
                    {
                        candidate_node
                    } else {
                        closest_node
                    }
                })
                .unwrap();
            (node, distance_squared_between_nodes(node, closest_node))
        })
        .collect::<Vec<_>>();

    closest_node_distances.sort_by(|a, b| a.1.cmp(&b.1));

    let mut graph = closest_node_distances
        .iter()
        .map(|n| n.0.clone())
        .collect::<Vec<_>>();

    fn distance_squared_between_nodes(node_a: &JunctionBox, node_b: &JunctionBox) -> usize {
        (node_a.x - node_b.x).pow(2) + (node_a.y - node_b.y).pow(2) + (node_a.z - node_b.z).pow(2)
    }

    let mut num_connections = 0;
    // Connect to nearest (that's not already connected to it)
    for i in 0..graph.len() {
        let node = &graph[i];
        // todo: cache distance?
        let closest_new_connection = graph
            .iter()
            .enumerate()
            .filter(|(j, n)| {
                &i != j && !n.connections.contains(&i) && !n.back_connections.contains(&i)
            })
            .reduce(|(closest_idx, closest_node), (j, candidate_node)| {
                if distance_squared_between_nodes(&node, candidate_node)
                    < distance_squared_between_nodes(&node, closest_node)
                {
                    (j, candidate_node)
                } else {
                    (closest_idx, closest_node)
                }
            });
        if let Some((closest_idx, _)) = closest_new_connection {
            graph[i].connections.push(closest_idx);
            graph[closest_idx].back_connections.push(i);
            num_connections += 1;
            if num_connections >= 10 {
                break;
            }
        }
    }

    fn dfs(
        graph: &Vec<JunctionBox>,
        start_idx: usize,
        visited: &mut Vec<bool>,
        visit_order: &mut Vec<usize>,
    ) {
        if visited[start_idx] {
            return;
        }
        visited[start_idx] = true;
        visit_order.push(start_idx);

        for &neighbor_idx in &graph[start_idx].connections {
            if !visited[neighbor_idx] {
                dfs(graph, neighbor_idx, visited, visit_order);
            }
        }
        for &neighbor_idx in &graph[start_idx].back_connections {
            if !visited[neighbor_idx] {
                dfs(graph, neighbor_idx, visited, visit_order);
            }
        }
    }

    dbg!(&graph);
    dbg!(
        &graph
            .iter()
            .enumerate()
            .map(|(i, node)| {
                let mut visit_order = Vec::new();
                dfs(
                    &graph,
                    i,
                    &mut graph.iter().map(|_| false).collect(),
                    &mut visit_order,
                );
                visit_order
            })
            .collect::<Vec<_>>()
    );
}

pub fn part2() {
    let input_str = include_str!("./real_input.txt");
}
