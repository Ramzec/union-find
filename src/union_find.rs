pub enum UnionFindType {
    QuickFind,
    QuickUnion,
    WeightQuickUnion,
}

pub struct UnionFind {
    items: Vec<u32>,
    weights: Vec<u32>,
    size: usize,
    stype: UnionFindType,
}

impl UnionFind {
    pub fn new(sz: u32, stype: UnionFindType) -> UnionFind {
        let mut cnt: u32 = 0;
        let mut items: Vec<u32> = Vec::new();
        let mut weights: Vec<u32> = Vec::new();

        while cnt < sz {
            items.push(cnt);
            weights.push(1);
            cnt = cnt + 1;
        }

        UnionFind {
            size: items.len(),
            items: items,
            weights: weights,
            stype: stype,
        }
    }
    
    pub fn print(&self) {
        println!("{:?}", self.items)
    }

    pub fn union(&mut self, idx1: usize, idx2: usize) {
        if !self.precheck(idx1, idx2) {
            return;
        }

        match self.stype {
            UnionFindType::QuickFind =>
                self.qf_union(idx1, idx2),
            UnionFindType::QuickUnion =>
                self.qu_union(idx1, idx2),
            UnionFindType::WeightQuickUnion =>
                self.wqu_union(idx1, idx2),
        }
    }

    pub fn connected(&self, idx1: usize, idx2: usize) -> bool {
        if !self.precheck(idx1, idx2) {
            return false;
        }

        match self.stype {
            UnionFindType::QuickFind =>
                self.qf_connected(idx1, idx2),
            UnionFindType::QuickUnion |
            UnionFindType::WeightQuickUnion =>
                self.qu_connected(idx1, idx2),
        }
    }
    
    // QuickUnion
    fn qu_connected(&self, idx1: usize, idx2: usize) -> bool {
        self.root(&idx1) == self.root(&idx2)
    }

    // QuickFind
    fn qf_connected(&self, idx1: usize, idx2: usize) -> bool {
        self.items[idx1] == self.items[idx2]
    }

    // WeightQuickUnion
    fn wqu_union(&mut self, idx1: usize, idx2: usize) {
        println!("WeightUnion Union of {} and {}", idx1, idx2);

        let idx1_root = self.root(&idx1);
        let idx2_root = self.root(&idx2);

        if idx1_root == idx2_root {
            return;
        }
        
        let weight = self.weights[idx1_root] +
            self.weights[idx2_root];

        if self.weights[idx2_root] > self.weights[idx1_root] {
            // Increase the weight of Root2
            self.weights[idx2_root] = weight;
            // Attach Root1 to Root2
            self.items[idx1_root] = self.items[idx2_root];
        } else {
            // Increase the weight of Root1
            self.weights[idx1_root] = weight;
            // Attach Root2 to Root1
            self.items[idx2_root] = self.items[idx1_root];
        }
    }
    
    // QuickUnion
    fn qu_union(&mut self, idx1: usize, idx2: usize) {
        println!("QuickUnion Union of {} and {}", idx1, idx2);

        let idx1_root = self.root(&idx1);
        let idx2_root = self.root(&idx2);

        if idx1_root == idx2_root {
            return;
        }

        self.items[idx1_root] = self.items[idx2_root];
    }

    // QuickFind
    fn qf_union(&mut self, idx1: usize, idx2: usize) {
        println!("QuickFind Union of {} and {}", idx1, idx2);

        let item1 = self.items[idx1];
        let item2 = self.items[idx2];

        for x in &mut self.items {
            if *x == item1 {
                *x = item2;
            }
        }
    }
   
    fn precheck(&self, idx1: usize, idx2: usize) -> bool {
        if idx1 > (self.size - 1) {
            println!("The item with index {} does not exist", idx1);
            return false;
        }

        if idx2 > (self.size - 1) {
            println!("The item with index {} does not exist", idx2);
            return false;
        }

        return true;
    }
    
    fn root(&self, idx: &usize) -> usize {
        let mut iidx = *idx;
        loop {
            let item = self.items[iidx];

            if item == iidx as u32 {
                return iidx;
            }

            iidx = item as usize;
        }
    }
}
