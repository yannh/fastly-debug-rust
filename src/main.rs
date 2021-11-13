use std::collections::HashMap;
use futures::executor::block_on;
use fastly_inspect::{fastly_inspect, FastlyInspect, GeoIP, FastlyInspectRequest};

fn main() {
    let popl: HashMap<String, String> = HashMap::new();
    let mut fi = FastlyInspect{
        geoip: GeoIP {
            ci: String::from(""),
            co: String::from(""),
            ct: String::from(""),
            st: String::from(""),
            c_ip: String::from(""),
            c_asn: String::from(""),
            c_asn_name: String::from(""),
            r_ip: String::from(""),
            r_asn: String::from(""),
            r_asn_name: String::from(""),
            r_ci: String::from(""),
            r_co: String::from(""),
            r_ct: String::from(""),
            r_st: String::from(""),
        },
        pop_latency: popl,
        request: FastlyInspectRequest{
            resolver_ip: String::from(""),
            resolver_as_name: String::from(""),
            resolver_as_number: String::from(""),
            resolver_country_code: String::from(""),
            client_ip: String::from(""),
            client_as_name: String::from(""),
            client_as_number: String::from(""),
            time: String::from(""),
            host: String::from(""),
            accept: String::from(""),
            useragent: String::from(""),
            acceptlanguage: String::from(""),
            acceptencoding: String::from(""),
            fastlyserverip: String::from(""),
            xff: String::from(""),
            datacenter: String::from(""),
            bandwidth_mbps: String::from(""),
            cwnd: 0,
            nexthop: String::from(""),
            rtt: 0.0,
            delta_retrans: 0,
            total_retrans: 0
        }
    };

    match block_on(fastly_inspect(String::from("http://127.0.0.1:7676"))) {
        Ok(res) => {
            fi = res;
        }
        Err(e) => eprintln!("{}", e),
    };

    match serde_json::to_string_pretty(&fi) {
        Ok(r) => println!("{}", r),
        Err(e) => eprintln!("{}", e),
    }
}
