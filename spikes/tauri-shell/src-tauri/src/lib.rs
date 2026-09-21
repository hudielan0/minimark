use minimark_atomic_save_spike::{
    fingerprint, load_recovery, save_if_unchanged, write_recovery, Fingerprint, RecoveryState,
    SaveError,
};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};
use tauri_plugin_dialog::DialogExt;

struct Session {
    path: PathBuf,
    fingerprint: Fingerprint,
}

#[derive(Default)]
struct SessionState(Mutex<Option<Session>>);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RecoveryPayload {
    draft: String,
    disk_changed: bool,
}

#[derive(Serialize)]
struct OpenedDocument {
    name: String,
    text: String,
    recovery: Option<RecoveryPayload>,
}

#[tauri::command]
async fn open_document(
    app: AppHandle,
    state: State<'_, SessionState>,
) -> Result<Option<OpenedDocument>, String> {
    let dialog_app = app.clone();
    let selected = tauri::async_runtime::spawn_blocking(move || {
        dialog_app
            .dialog()
            .file()
            .add_filter("Markdown", &["md", "markdown"])
            .blocking_pick_file()
    })
    .await
    .map_err(|error| error.to_string())?;

    let Some(selected) = selected else {
        return Ok(None);
    };
    let path = selected.into_path().map_err(|error| error.to_string())?;
    let text = fs::read_to_string(&path).map_err(|error| error.to_string())?;
    let current_fingerprint = fingerprint(&path).map_err(|error| error.to_string())?;
    let recovery = load_recovery(&recovery_directory(&app)?, &path)
        .map_err(|error| error.to_string())?
        .map(|offer| RecoveryPayload {
            draft: offer.draft,
            disk_changed: offer.disk_state == RecoveryState::DiskChanged,
        });
    let name = path
        .file_name()
        .map(|value| value.to_string_lossy().into_owned())
        .unwrap_or_else(|| "Markdown document".to_owned());

    *state.0.lock().map_err(|error| error.to_string())? = Some(Session {
        path,
        fingerprint: current_fingerprint,
    });

    Ok(Some(OpenedDocument {
        name,
        text,
        recovery,
    }))
}

#[tauri::command]
fn write_draft(app: AppHandle, state: State<'_, SessionState>, text: String) -> Result<(), String> {
    let session = state.0.lock().map_err(|error| error.to_string())?;
    let session = session
        .as_ref()
        .ok_or_else(|| "请先打开一个 Markdown 文件".to_owned())?;
    write_recovery(
        &recovery_directory(&app)?,
        &session.path,
        &session.fingerprint,
        &text,
    )
    .map_err(|error| error.to_string())
}

#[tauri::command]
fn save_document(state: State<'_, SessionState>, text: String) -> Result<(), String> {
    let mut session = state.0.lock().map_err(|error| error.to_string())?;
    let session = session
        .as_mut()
        .ok_or_else(|| "请先打开一个 Markdown 文件".to_owned())?;
    session.fingerprint = save_if_unchanged(&session.path, &session.fingerprint, &text)
        .map_err(display_save_error)?;
    Ok(())
}

fn display_save_error(error: SaveError) -> String {
    match error {
        SaveError::ExternalChange => "磁盘上的文件已被其他程序修改；原文件没有被覆盖".to_owned(),
        SaveError::Io(error) => format!("保存失败：{error}"),
    }
}

fn recovery_directory(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_local_data_dir()
        .map(|path| path.join("recovery"))
        .map_err(|error| error.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(SessionState::default())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            open_document,
            write_draft,
            save_document
        ])
        .run(tauri::generate_context!())
        .expect("error while running MiniMark Phase 1");
}
