mod load_balancer;

use load_balancer::LoadBalancer;

fn main() {
    let lb = LoadBalancer::new("192.168.1.137:8080", vec!["192.168.1.137:8000", "192.168.1.137:8001"].iter().map(|s| s.to_string()).collect());
    lb.run();
}

