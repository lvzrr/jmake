
pub struct  CONFIG
{
    pub pre:            Vec<String>,
    pub bin:            String,
    pub src:            String,
    pub lib:            String,
    pub classpath:      String,
    pub post:           Vec<String>,
    pub comp_flags:     String,
    pub runner_flags:   String,
    pub args:           String,
    pub cache:          String,
    pub threads:        usize,
}

