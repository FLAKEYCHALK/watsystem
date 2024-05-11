use sysinfo::System; 
mod interface;


fn main() {
   let mut sys = System::new_all();
   sys.refresh_all();

   let mem = sys.total_memory() / (1024 * 1024 * 1024); 
   let used_mem = sys.used_memory() / (1024 * 1024 * 1024);

   println!("{}", interface::PROGRAM_TITLE);

   println!("┯━━━━━━━━━━━━━━━ ●●● ━━━━━━━━━━━━━━━┯");
   println!("Name:          {:?}", get_computer_name());
   println!("Ram:           {}/{} GB", used_mem, mem);
   println!("OS:            {:?}", get_os_name());
   println!("Kernel:        {:?}", get_kernel_version());
   println!("┷━━━━━━━━━━━━━━━ ●●● ━━━━━━━━━━━━━━━┷");

}

fn get_computer_name() -> String {
   System::host_name().unwrap_or_else(|| "Unknown".to_string())
}

fn get_os_name() -> String {
   System::long_os_version().unwrap_or_else(|| "Unknown".to_string())
}

fn get_kernel_version() -> String {
   System::kernel_version().unwrap_or_else(|| "Unknown".to_string())
}