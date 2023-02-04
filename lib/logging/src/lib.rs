use std::io::Write;
use bevy::prelude::*;
use tracing_subscriber::layer::SubscriberExt;

#[derive(Default)]
pub struct BloccLoggPlugin(bevy::log::LogPlugin);

const LOG_DIRECTORY: &str = "log";
const ARCHIVE_NAME: &str = "archive";

impl Plugin for BloccLoggPlugin {
    fn build(&self, app: &mut App) {
        prepare_directory();

        tracing_log::LogTracer::init()
            .expect("Failed to initialize LogTracer");

        #[allow(unused_must_use)] {
            enable_ansi_support::enable_ansi_support();
        }

        let old_handler = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |infos| {
            println!("{}", tracing_error::SpanTrace::capture());
            old_handler(infos);
        }));

        let subscriber = tracing_subscriber::registry::Registry::default();

        let filter_layer = tracing_subscriber::EnvFilter::try_from_default_env()
            .or_else(|_| {
                tracing_subscriber::EnvFilter::try_new(
                    format!("{},{}", self.0.level, self.0.filter)
                )
            })
            .unwrap();

        let subscriber = subscriber.with(filter_layer);

        let error_trace_layer = tracing_error::ErrorLayer::default();

        let subscriber = subscriber.with(error_trace_layer);

        let stdout_layer = tracing_subscriber::fmt::Layer::default();

        let subscriber = subscriber.with(stdout_layer);

        #[cfg(feature = "tracy")]
        let subscriber = subscriber.with(
            tracing_tracy::TracyLayer::new()
        );

        let file_layer = {
            let filename = format!(
                "{}.log",
                std::time::UNIX_EPOCH.elapsed().unwrap().as_secs()
            );

            let writer = tracing_appender::rolling::never(
                LOG_DIRECTORY,
                filename
            );

            let (non_blocking_appender, guard) =
                tracing_appender::non_blocking(writer);

            app.insert_resource(LogFileWorkerGuard(guard));

            tracing_subscriber::fmt::Layer::default()
                .with_ansi(false)
                .with_writer(non_blocking_appender)
        };

        let subscriber = subscriber.with(file_layer);

        bevy::utils::tracing::subscriber::set_global_default(subscriber)
            .expect("Failed to set global subscriber");
    }
}

fn prepare_directory() {
    let log_path = std::path::Path::new(LOG_DIRECTORY);

    if !log_path.exists() {
        std::fs::create_dir(log_path)
            .expect(&format!("Failed to create logging directory `{LOG_DIRECTORY}`"));
    }

    let file_path = {
        let mut file_path = log_path.join(ARCHIVE_NAME);
        file_path.set_extension("zip");
        file_path
    };

    let file = std::fs::File::options()
        .read(true)
        .write(true)
        .open(&file_path);

    let mut archive = if let Ok(file) = file {
        zip::ZipWriter::new_append(file)
            .expect(&format!("Failed to open `{:?}` as zip archive", file_path))
    } else {
        let file = std::fs::File::create("log/archive.zip")
            .expect(&format!("Failed to create a new archive as {:?}", file_path));
        zip::ZipWriter::new(file)
    };

    for entry in std::fs::read_dir(log_path).unwrap().flatten() {
        let path = entry.path();
        if path.extension() == Some(std::ffi::OsStr::new("log")) {
            let data = std::fs::read(&path)
                .expect(&format!("Failed to read log file {:?} for compression", path));

            archive.start_file(
                entry.file_name().to_string_lossy(),
                zip::write::FileOptions::default()
            ).expect(&format!("Failed to start log file {:?} in archive", path));

            archive.write_all(&data)
                .expect(&format!("Failed to write data of {:?} to archive", path));

            std::fs::remove_file(&path)
                .expect(&format!("Failed to remove file {:?} after write", path));
        }
    }

    archive.finish().expect(&format!("Failed to finish writing to archive {:?}", file_path));
}

#[derive(Resource)]
struct LogFileWorkerGuard(tracing_appender::non_blocking::WorkerGuard);