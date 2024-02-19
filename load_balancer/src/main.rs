mod load_balancer;

use load_balancer::LoadBalancer;

fn main() {
    let lb = LoadBalancer::new("172.20.10.3:8080", vec!["172.20.10.3:8000", "172.20.10.3:8001"].iter().map(|s| s.to_string()).collect());
    lb.run();
}

