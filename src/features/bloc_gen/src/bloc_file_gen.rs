//

use crate::lib::file_helper;

pub async fn init_bloc_gen() {
    // generate bloc state
    let file_result =
        file_helper::write_to_file::write_to_file("./bloc", "test_bloc.dart", "  ").await;

    match file_result {
        Ok(_) => {}
        Err(e) => tracing::error!("{:?}", e),
    }

    // generate bloc event
    // generate bloc

    // save all 3 file
}
