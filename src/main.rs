extern crate x86;
extern crate time;
extern crate raw_cpuid;
extern crate perfcnt;
extern crate docopt;
extern crate rustc_serialize;

use std::thread;
use std::time::Duration;

mod perf_counters;

use perf_counters::cpucounters::consumer::CountersConsumer;

use docopt::Docopt;
use std::process::{Command, Stdio};


//The Docopt usage string.
const USAGE: &'static str = "
Usage:  pcm-reader  [-p] --pid=<processPid>
        pcm-reader  [-a] --app=<app2Start> --args2app=<args>
Options:
    -p,    --pid=<processPid>     	Process pid to monitor.
    -a,    --app=<app2Start>     	Application to monitor
    --args2app=<args>         		Arguments for the Application to monitor
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_pid: i32,
    flag_app: Option<String>,
    flag_args2app: String,
} 

fn main() {
	
	    /// Collect command line arguments
	    ///
	    let args: Args = Docopt::new(USAGE)
	        .and_then(|d| d.decode())
	        .unwrap_or_else(|e| e.exit());
		
		
		let mut perf_meter = match args.flag_app {
			Some(path_app) => {
				let app_args: Vec<&str>=args.flag_args2app.split_whitespace().collect();
		        let mut app_process = Some(Command::new(path_app)
		            .args(app_args.as_ref())
		            .stdout(Stdio::piped())
		            .spawn()
		            .expect("Failed to execute Application!")); 
		        
		        let pid_app = app_process.as_mut().unwrap().id();
		        
			    CountersConsumer::new(pid_app as i32)
			},
			None => CountersConsumer::new(args.flag_pid),
		};
		
        
        let mut initial_counters = perf_meter.get_current_counters();


	    loop{
	    		
	        let current_counters = perf_meter.get_current_counters();
	
	        println!("Current {:?}",current_counters);
	        let cpu_time =
	            perf_meter.get_cpu_exec_time(initial_counters.clone(), current_counters.clone());
	        let ipc = perf_meter.get_core_ipc(initial_counters.clone(), current_counters.clone());
	        let ipc_util =
	            perf_meter.get_ipc_utilization(initial_counters.clone(), current_counters.clone());
	        let core_utilization =
	            perf_meter.get_core_utilization(initial_counters.clone(), current_counters);
	
	
			
	
	      	println!("CPU Time: {:.4} - IPC: {:.4} - IPC Utilization: {:.2}% - worker_nr \
		                  Utilization: {:.2}%",
		                 cpu_time,
		                 ipc,
		                 ipc_util,
		                 core_utilization);
  		    thread::sleep(Duration::from_millis(500));	
	      	
	      	}
	    

          
}
        