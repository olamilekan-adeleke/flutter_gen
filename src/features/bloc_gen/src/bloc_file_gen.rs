use crate::{
    features::bloc_gen::data::bloc_base_text::BlocBaseText,
    lib::{error::AppError, file_helper::write_to_file::write_to_file},
};

struct BlocFileData {
    file_name: String,
    file_content: String,
}

impl BlocFileData {
    fn new(file_name: String, file_content: String) -> BlocFileData {
        BlocFileData {
            file_name,
            file_content,
        }
    }

    fn get_data(class_name: &str) -> Vec<BlocFileData> {
        let state_file_content = BlocBaseText::bloc_state(class_name);
        let state_file = &format!("{}_state.dart", class_name);

        let event_file_content = BlocBaseText::bloc_event(class_name);
        let event_file = &format!("{}_event.dart", class_name);

        let bloc_file_content = BlocBaseText::bloc_main_class(class_name);
        let bloc_file = &format!("{}_bloc.dart", class_name);

        vec![
            BlocFileData::new(state_file.to_string(), state_file_content),
            BlocFileData::new(event_file.to_string(), event_file_content),
            BlocFileData::new(bloc_file.to_string(), bloc_file_content),
        ]
    }
}

pub async fn init_bloc_gen() -> anyhow::Result<()> {
    let path = "./bloc";
    let class_name = "test".to_lowercase();

    // generate bloc file
    let data = BlocFileData::get_data(&class_name);
    for data in data.iter() {
        let file_result = write_to_file(path, data.file_name.as_str(), &data.file_content).await;

        if file_result.is_err() {
            tracing::error!("Error: create bloc_state fail \n{:?}", file_result);
            return Err(anyhow::anyhow!(AppError::FileCreationError(
                file_result.unwrap_err().to_string()
            )));
        }
    }

    Ok(())
}
