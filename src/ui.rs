use anyhow;
#[allow(unused_imports)]
use inquire::{min_length, Confirm, CustomType, MultiSelect, Password, Select, Text};

use crate::args::{Commands, DislodgeParams, DownloadParams, EmbedParams};

pub async fn enrich_arguments(args: Option<Commands>) -> anyhow::Result<Commands> {
    Ok(match args {
        Some(Commands::Embed(embed_args)) => {
            Commands::Embed(enrich_embed_params(embed_args).await?)
        }
        Some(Commands::Download(download_args)) => {
            Commands::Download(enrich_download_params(download_args).await?)
        }
        Some(Commands::Dislodge(dislodge_args)) => {
            Commands::Dislodge(enrich_dislodge_params(dislodge_args).await?)
        }
        None => {
            let options = vec!["Encrypt", "Download", "Decrypt"];

            let modes = Select::new("Choose One", options)
                .with_help_message("Encrypt: Encrypt any file into a video,\n Download: Download files stored on YouTube,\n Decrypt: Return files from an encrypted video")
                .prompt()
                .unwrap();

            match modes {
                "Encrypt" => Commands::Embed(enrich_embed_params(EmbedParams::default()).await?),
                "Download" => {
                    Commands::Download(enrich_download_params(DownloadParams::default()).await?)
                }
                "Decrypt" => {
                    Commands::Dislodge(enrich_dislodge_params(DislodgeParams::default()).await?)
                }
                _ => unreachable!(),
            }
        }
    })
}

async fn enrich_embed_params(mut args: EmbedParams) -> anyhow::Result<EmbedParams> {
    if args.in_path.is_none() {
        let path = Text::new("Enter the path of your file :")
            .with_default("src/tests/test.txt")
            .prompt()
            .unwrap();
        args.in_path = Some(path);
    }

    
    if args.mode.is_none()
        && args.block_size.is_none()
        && args.threads.is_none()
        && args.fps.is_none()
        && args.resolution.is_none()
    {
        let presets = vec![
            "Optimal compression resistance",
            "Paranoid compression resistance",
            "Maximum efficiency",
            "Custom",
        ];
        let preset = Select::new("You can use one of the existing presets or custom settings", presets.clone())
            .with_help_message("Any amount of compression on Maximum Efficiency will corrupt all your hopes and dreams")
            .prompt()
            .unwrap();

        match preset {
            "Maximum efficiency" => {
                args.preset = Some(crate::args::EmbedPreset::MaxEfficiency);
                return Ok(args);
            }
            "Optimal compression resistance" => {
                args.preset = Some(crate::args::EmbedPreset::Optimal);
                return Ok(args);
            }
            "Paranoid compression resistance" => {
                args.preset = Some(crate::args::EmbedPreset::Paranoid);
                return Ok(args);
            }
            _ => (),
        }
    }


    if args.mode.is_none() {
        let out_modes = vec!["Colored", "B/W (Binary)"];
        let out_mode = Select::new("Pick how data will be embedded", out_modes.clone())
            .with_help_message("Colored mode is useless.")
            .prompt()
            .unwrap();
        args.mode = Some(match out_mode {
            "Colored" => crate::args::EmbedOutputMode::Colored,
            "B/W (Binary)" => crate::args::EmbedOutputMode::Binary,
            _ => unreachable!(),
        });
    }

    if args.block_size.is_none() {
        let size = CustomType::<i32>::new("What size should the blocks be ?")
            .with_error_message("Please type a valid number")
            .with_help_message("Bigger blocks are more resistant to compression, I recommend 2-5.")
            .with_default(2)
            .prompt()?;
        args.block_size = Some(size);
    }

    if args.threads.is_none() {
        let threads = CustomType::<usize>::new("How many threads to dedicate for processing ?")
            .with_error_message("Please type a valid number")
            .with_help_message("The more threads, the merrier")
            .with_default(8)
            .prompt()?;
        args.threads = Some(threads);
    }

    if args.fps.is_none() {
        let fps = CustomType::<i32>::new("Enter the fps :")
            .with_error_message("Please type a valid number")
            .with_help_message("Decreasing fps may decrease chance of compression. ~10fps works")
            .with_default(10)
            .prompt()
            .expect("Invalid fps");
        args.fps = Some(fps);
    }

    let resolutions = vec!["144p", "240p", "360p", "480p", "720p"];

    if args.resolution.is_none() {
        
        let resolution = Select::new("Select a resolution", resolutions)
            .with_help_message("Recommended 720p, as the resolution won't affect compression")
            .prompt()
            .unwrap();
        args.resolution = Some(resolution.to_string());
    }

    Ok(args)
}

async fn enrich_download_params(mut args: DownloadParams) -> anyhow::Result<DownloadParams> {
    if args.url.is_none() {
        let url = Text::new("Enter the url of th video :")
            .prompt()
            .unwrap();
        args.url = Some(url);
    }
    Ok(args)
}


async fn enrich_dislodge_params(mut args: DislodgeParams) -> anyhow::Result<DislodgeParams> {
    if args.in_path.is_none() {
        let in_path = Text::new("Enter the path of the Encrypted video :")
            .with_default("output.avi")
            .prompt()
            .unwrap();
        args.in_path = Some(in_path);
    }

    if args.out_path.is_none() {
        let out_path = Text::new("Output destination path :")
            .with_help_message("Please include name of file and extension")
            .prompt()
            .unwrap();
        args.out_path = Some(out_path);
    }


    Ok(args)
}
