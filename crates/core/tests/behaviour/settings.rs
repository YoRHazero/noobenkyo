//! Study settings: read, replaced, and restored from the defaults.

use noobenkyo_lib::commands::settings::{
    study_default_settings, study_settings, update_study_settings,
};
use study::StudySettings;

use crate::support::TestApp;

#[tokio::test]
async fn settings_start_as_the_defaults_and_are_replaced_as_a_whole() {
    let app = TestApp::fresh().await;
    assert_eq!(study_default_settings(), StudySettings::default());
    assert_eq!(
        study_settings(app.state()).await.unwrap(),
        study_default_settings()
    );

    let mut smaller = study_default_settings();
    smaller.learn.capacity = 3;
    update_study_settings(app.state(), smaller.clone())
        .await
        .unwrap();
    assert_eq!(study_settings(app.state()).await.unwrap(), smaller);

    update_study_settings(app.state(), study_default_settings())
        .await
        .unwrap();
    assert_eq!(
        study_settings(app.state()).await.unwrap(),
        study_default_settings()
    );
}

#[tokio::test]
async fn invalid_settings_are_rejected_with_the_reason() {
    let app = TestApp::fresh().await;
    let mut invalid = study_default_settings();
    invalid.learn.capacity = 0;
    let error = update_study_settings(app.state(), invalid)
        .await
        .unwrap_err();
    assert_eq!(error.message, "设置不合法：学习区的容量至少为 1");
    assert_eq!(
        study_settings(app.state()).await.unwrap(),
        study_default_settings()
    );
}
