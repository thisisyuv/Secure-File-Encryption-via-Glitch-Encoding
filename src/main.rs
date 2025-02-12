mod args;
mod embedsource;
mod etcher;
mod run_tasks;
mod settings;
mod timer;
mod ui;

use clap::Parser;

use crate::args::Arguments;

//Make RGB a struct
//Make it calculate how much data is jammed in 1 frame for user
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Secure File Encryption.");
    println!("This tool allows you to encrypt any file into a compression-resistant video.");
    println!("\nHow to use:");
    println!("1. Zip all the files you will be uploading");
    println!("2. Use the embed option on the archive (THE VIDEO WILL BE SEVERAL TIMES LARGER THAN THE FILE, 4x in case of optimal compression resistance preset)");
    println!("3. Use the download option to get the video from Youtube");
    println!("5. Use the dislodge option to get your files back from the downloaded video");
    println!("6. PROFIT\n");

    let mut args = Arguments::parse();
    let new_command = ui::enrich_arguments(args.command).await?;
    args.command = Some(new_command);

    run_tasks::run_by_arguments(args).await?;
    Ok(())
}
