use jni;

#[cfg(unix)]
pub const SHELL: &str = "sh";
#[cfg(unix)]
pub const FLAG: &str = "-c";

#[cfg(windows)]
pub const SHELL: &str = "cmd";
#[cfg(windows)]
pub const FLAG: &str = "/C";

pub struct  CONFIG
{
    pub pre:            Vec<String>,
    pub bin:            String,
    pub src:            String,
    pub test:            String,
    pub lib:            String,
    pub classpath:      String,
    pub post:           Vec<String>,
    pub comp_flags:     String,
    pub run_args:       Vec<String>,
    pub cache:          String,
    pub threads:        usize,
    pub jvm_version:    jni::JNIVersion,
    pub jvm_options:    Vec<String>,
}
