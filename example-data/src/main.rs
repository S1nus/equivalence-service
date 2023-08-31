use nmt_rs::{
    CelestiaNmt as Nmt, 
    NamespaceId,
};
use rand::{Rng, thread_rng};

fn random_leaf() -> [u8; 512] {
    let mut rng = thread_rng();
    let mut random_array: [u8; 512] = [0; 512];
    rng.fill(&mut random_array);
    random_array
}

fn main() {
    let mut tree = Nmt::new();

    let mut ns1: NamespaceId<29> = NamespaceId([0; 29]);
    ns1.0[..1].copy_from_slice(&1u8.to_le_bytes());

    let mut ns2: NamespaceId<29> = NamespaceId([0; 29]);
    ns2.0[..1].copy_from_slice(&2u8.to_le_bytes());

    let mut ns3: NamespaceId<29> = NamespaceId([0; 29]);
    ns3.0[..1].copy_from_slice(&3u8.to_le_bytes());

    for _ in 0..10 {
        tree.push_leaf(&random_leaf()[..], ns1)
            .expect("Failed to push leaf");
    }
    for _ in 10..20 {
        tree.push_leaf(&random_leaf()[..], ns2)
            .expect("Failed to push leaf");
    }
    for _ in 20..31 {
        tree.push_leaf(&random_leaf()[..], ns3)
            .expect("Failed to push leaf");
    }
    let leaves = tree.leaves();
    println!("num leaves: {}", leaves.len());

    let mut count = 0;
    for chunk in leaves.chunks(2) {
        let min = chunk[0].hash.min_namespace().0[0];
        if chunk.len() > 1 {
            let max = chunk[1].hash.max_namespace().0[0];
            println!("min: {:?}, max: {:?}", min, max);
        }
        else {
           println!("not full"); 
        }
        count += 1;
    }
    println!("number of layer 2 leaves: {}", count);
}