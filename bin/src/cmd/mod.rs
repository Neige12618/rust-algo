pub mod fetch;
pub mod submit;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    /// 获取今日题目并写入对应文件(alias: f)
    #[command(alias = "f")]
    Fetch(FetchArgs),
    /// 提交今日题目并判题(alias: s)
    #[command(alias = "s")]
    Submit(SubmitArgs),
}

#[derive(clap::Args, Debug)]
pub struct FetchArgs {
    #[arg(short, long)]
    filter: Option<String>,
}

#[derive(clap::Args, Debug)]
pub struct SubmitArgs {
    #[arg(short, long)]
    id: Option<String>,
}
