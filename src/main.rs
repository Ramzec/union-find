//mod quick_find;
//mod quick_union;

mod union_find;

fn main() {
    let mut uf1 = union_find::UnionFind::new(10,
        union_find::UnionFindType::QuickFind);
    let mut uf2 = union_find::UnionFind::new(10,
        union_find::UnionFindType::QuickUnion);
    let mut uf3 = union_find::UnionFind::new(10,
        union_find::UnionFindType::WeightQuickUnion);

    println!("QuickFind UnionFind");

    uf1.print();
    uf1.union(1, 9);
    uf1.print();
    uf1.union(9, 2);
    uf1.print();
    
    println!("Connected {}\n\n", uf1.connected(1, 2));

    println!("QuickUnion UnionFind");

    uf2.print();
    uf2.union(1, 9);
    uf2.print();
    uf2.union(9, 2);
    uf2.print();

    println!("Connected {}\n\n", uf2.connected(1, 2));
    
    println!("WeightQuickUnion UnionFind");

    uf3.print();
    uf3.union(1, 9);
    uf3.print();
    uf3.union(9, 2);
    uf3.print();

    println!("Connected {}", uf3.connected(1, 2));
}
